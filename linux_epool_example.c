#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/epoll.h>

#define MAX_EVENTS 10

int main(void) {
  int listen_fd = socket(AF_INET, SOCK_STREAM, 0); // Создаём серверный сокет

  int options = 1; // Разрешаем переиспользование адреса
  setsockopt(listen_fd, SOL_SOCKET, SO_REUSEADDR, &options, sizeof(options));

  // Привязывает сокет к 8081 порту и начинаем слушать
  struct sockaddr_in addr;
  addr.sin_family = AF_INET;
  addr.sin_addr.s_addr = INADDR_ANY;
  addr.sin_port = htons(8081);
  bind(listen_fd, (struct sockaddr *)&addr, sizeof(addr));
  listen(listen_fd, SOMAXCONN);

  // Создаём мультиплексор неблокируюшего ввода/вывода epoll
  int epoll_fd = epoll_create1(0);

  // Добавляем серверный сокет в epoll
  struct epoll_event ev;
  ev.events = EPOLLIN;
  ev.data.fd = listen_fd;
  epoll_ctl(epoll_fd, EPOLL_CTL_ADD, listen_fd, &ev);

  // В цикле ждём пояления дискриптора готового к работе
  while (1) {
    struct epoll_event events[MAX_EVENTS];
    // Ожидаем появления готовых дескрипторов ввода/вывода
    int ready_num = epoll_wait(epoll_fd, events, MAX_EVENTS, -1);

    // Итерируемся по всем готовым к обработке дескрипторам
    for (int i = 0; i < ready_num; i++) {
      if (events[i].data.fd == listen_fd) {
        // Если готовый дескриптор - наш серверный сокет,
        // значит поступило новое сетевое соединение.
        // Принимаем его и добавляем новое соединение в epoll
        struct sockaddr_in client_addr;
        socklen_t client_len = sizeof(client_addr);
        int conn_fd = accept(listen_fd, (struct sockaddr *)&client_addr, &client_len);

        printf("New: %s:%d\n", inet_ntoa(client_addr.sin_addr), ntohs(client_addr.sin_port));

        ev.events = EPOLLIN;
        ev.data.fd = conn_fd;
        epoll_ctl(epoll_fd, EPOLL_CTL_ADD, conn_fd, &ev);
      } else {
        // Иначе, это ввод/вывод от клиента
        int client_fd = events[i].data.fd;
        char buf[1024];
        ssize_t read_bytes = read(client_fd, buf, sizeof(buf));
        if (read_bytes <= 0) {
          // Если от клиента считано всё, то закрываем
          // соединение и удаляем его дескриптор из epoll
          close(client_fd);
          epoll_ctl(epoll_fd, EPOLL_CTL_DEL, client_fd, NULL);
        } else {
          // Иначе сразу пишем считанные байты обратно клиенту
          write(client_fd, buf, read_bytes);
        }
      }
    }
  }

  close(listen_fd);
  close(epoll_fd);
  return 0;
}
