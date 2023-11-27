# docker build -t bonus .
# docker run --rm --name bonus bonus
FROM alpine AS build
RUN apk --no-cache add nasm gcc
WORKDIR /build
COPY script.asm .
RUN nasm -f elf64 script.asm
RUN ld script.o -o script 
CMD ["/build/script"]

FROM scratch
COPY --from=build /build/script /script
CMD ["/script"]