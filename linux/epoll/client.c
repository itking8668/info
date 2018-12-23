#include<stdio.h>
#include<arpa/inet.h>
#include<sys/epoll.h>
#include<unistd.h>
#include<ctype.h>

int main(int argc, char *argv[])
{
    int iRet = 0;
    int iSockFd = socket(AF_INET,SOCK_STREAM,0);
    int uServerPort = 8000;
    int ireuseadd_on = 1;//支持端口复用
    unsigned long ul = 1;
    char buffer[] = {"i am lower"};
    char rbuffer[1024] = {0};

    struct sockaddr_in addr;
    bzero(&addr, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(uServerPort);
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");

    iRet = setsockopt(iSockFd, SOL_SOCKET, SO_REUSEADDR, &ireuseadd_on, sizeof(ireuseadd_on));  
    // ioctl(iSockFd, FIONBIO, &ul); //设置为非阻塞模式  

    iRet = connect(iSockFd, (const struct sockaddr*)&addr, sizeof(addr));
    if (iRet !=0){
        printf("connect ret : %d\n", iRet);
    }
    iRet = send(iSockFd, buffer, strlen(buffer), MSG_NOSIGNAL);
    if (iRet != strlen(buffer)){
        printf("send length : %d/%d\n",iRet, strlen(buffer));
    }

    sleep(1);

    iRet = recv(iSockFd, rbuffer, 1024, 0);
    printf("recv buffer : %s\n",rbuffer);

    return 0; 
}  