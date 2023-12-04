# docker build -t bonus .
# docker run --rm --name bonus bonus
FROM alpine AS build
RUN apk --no-cache add as gcc
WORKDIR /build
COPY script.asm .
RUN as script.asm -o script.o
RUN ld script.o -o script

FROM scratch
COPY --from=build /build/script /script
CMD ["/script"]