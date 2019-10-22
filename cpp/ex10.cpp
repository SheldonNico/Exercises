#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <iostream>
#include <cstring>


int main(int argc, char* argv[]) {
  struct addrinfo hints, *res, *p;
  int status;
  char ipstr[INET6_ADDRSTRLEN];
  if (argc != 2) {
    std::cerr << "usage: show ip hostname\n";
    return 1;
  }

  memset(&hints, 0, sizeof_hints);


  return 0;
}
