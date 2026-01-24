use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, Arc, Mutex,
    },
    task::{Context, Poll, Wake},
    thread::{self, sleep},
    time::Duration,
};

// Псевдоним для "приколотого" бокса, содержащего фьючер.
// Нам придётся хранить фьючеры как объекты Pin<Box<dyn Future>>
// чтобы иметь возможность вызывать poll, который требует Pin<&mut Self>
type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

// Обёртка для фьючера, сгенерированного комипилятором из async функции
struct SpawnedTask {
    id: u64,
    future: Mutex<Option<BoxFuture>>,
}

// Реализация фьючера, которая делает паузу (как функция thread::sleep)
// В функции main вы будем создавать такой фьючер.
pub struct Sleep {
    interval: Duration,
    is_ready: Arc<AtomicBool>,
}

impl Sleep {
    pub fn new(interval: Duration) -> Sleep {
        Sleep {
            interval,
            is_ready: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_ready.load(Ordering::SeqCst) {
            return Poll::Ready(());
        } else {
            let waker = cx.waker().clone();
            let ready_flag = self.is_ready.clone();
            let interval_to_sleep = self.interval.clone();
            // самая примитивная реализация - стартовать новый поток для ожидания
            thread::spawn(move || {
                sleep(interval_to_sleep);
                ready_flag.store(true, Ordering::SeqCst);
                // извещаем экзекьютор об окончании работы фьючера
                waker.wake();
            });
            Poll::Pending
        }
    }
}

// Интерфейс для работы с экзекьютором
pub struct Executor {
    runtime: ExecutorRuntime,
    last_task_id: AtomicU64,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            runtime: ExecutorRuntime::new(),
            last_task_id: AtomicU64::new(1),
        }
    }
    // Используется для добавления async функции в очередь экзекьютора
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(SpawnedTask {
            id: self.last_task_id.fetch_add(1, Ordering::SeqCst),
            future: Mutex::new(Some(Box::pin(fut))),
        });
        let _ = self.runtime.task_producer.send(task);
    }

    // Запускает вычисление фьючеров (файберов) из очереди экзекьютора
    pub fn exec_blocking(&mut self) {
        self.runtime.run();
    }
}

// Инкапсулирует код для непосредственного вычисление фьючеров
pub struct ExecutorRuntime {
    // Sender, который выдаётся другим компонентам (Executor и Taks)
    // чтобы они могли добавления async функции в очередь
    task_producer: mpsc::Sender<Arc<SpawnedTask>>,
    // Receiver используемый рантаймом для извлечения следующей async-функции
    task_queue: mpsc::Receiver<Arc<SpawnedTask>>,
    // Хранилище для фьючеров, которые при первом вызове poll вернули
    // Poll::Pending. Нужно, чтобы не завершить работу до того как выполнены
    // все фьючеры.
    task_pending: HashSet<u64>,
}

impl ExecutorRuntime {
    pub fn new() -> ExecutorRuntime {
        let (sender, receiver) = mpsc::channel::<Arc<SpawnedTask>>();
        ExecutorRuntime {
            task_producer: sender,
            task_queue: receiver,
            task_pending: HashSet::new(),
        }
    }

    // Запуск исполнения фьючеров
    pub fn run(&mut self) {
        loop {
            match self.task_queue.recv_timeout(Duration::from_secs(1)) {
                Ok(task) => self.process_task(task),
                Err(_) =>
                // Если очередь фьючеров пуста, и нет фьючеров, которые
                // в процессе исполнения, тогда обработка завершается
                {
                    if self.task_pending.is_empty() {
                        break;
                    }
                }
            }
        }
    }

    fn process_task(&mut self, task: Arc<SpawnedTask>) {
        let mut future_guard = task.future.lock().unwrap();
        // Извлекаем объект Pin<Box<dyn Future>> из таска, потому что для
        // вызова poll нужен именно объект (по значению), а не ссылка
        let Some(mut fut) = future_guard.take() else {
            return; // already finished
        };

        // Создаём Waker на случай, если фьючер не сможет выполниться сразу
        // и вернёт Poll::Pending.
        let spawned_task_waker = SpawnedTaskWaker {
            task: task.clone(),
            sender: self.task_producer.clone(),
        };
        let waker = Arc::new(spawned_task_waker).into();
        let mut cx = Context::from_waker(&waker);

        // Выполняем фьючер
        let poll_result = fut.as_mut().poll(&mut cx);

        match poll_result {
            Poll::Pending => {
                // Засовываем фьючер обратно в таск, так как этот таск придётся
                // обрабатывать снова, после того как фьючер вызове waker
                *future_guard = Some(fut);
                // Запоминаем, что таск с таким ID выполняется на фоне
                self.task_pending.insert(task.id);
            }
            Poll::Ready(()) => {
                // Удаляем (если надо) ID таска из списка тасков,
                // выполняющихся на фоне
                self.task_pending.remove(&task.id);
            }
        }
    }
}

// Простейший Waker, который просто еще раз добавляет таск в очередь рантайма
struct SpawnedTaskWaker {
    sender: mpsc::Sender<Arc<SpawnedTask>>,
    task: Arc<SpawnedTask>,
}

impl Wake for SpawnedTaskWaker {
    fn wake(self: Arc<Self>) {
        let _ = self.sender.send(self.task.clone());
    }
}
