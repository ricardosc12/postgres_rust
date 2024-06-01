FROM rust:latest

WORKDIR /home

COPY . .

RUN cargo build --release

ENTRYPOINT ["cargo", "run", "--release"]

# Exponha a porta em que o servidor web estará escutando
# EXPOSE 8080

