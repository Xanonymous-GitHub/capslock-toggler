#include <sys/ioctl.h>
#include <fcntl.h>
#include <unistd.h>
#include <iostream>
#include <stdlib.h>

int main(int argc, const char *argv[]) {
    const int fd_console = open("/dev/console", O_WRONLY);
    if (fd_console == -1) {
        std::cerr << "Error opening console file descriptor\n";
        exit(-1);
    }

    // turn on caps lock
    ioctl(fd_console, 0x4B32, 0x04);

    // turn on num block
    ioctl(fd_console, 0x4B32, 0x02);

//    // turn off
//    ioctl(fd_console, 0x4B32, 0x0);

    close(fd_console);
    return 0;
}
