# Usa una imagen base oficial de Rust
FROM rust:latest AS builder

# Establece las variables de entorno para Rocket
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

# Establece el directorio de trabajo
WORKDIR /app

# Copia los archivos de la aplicación
COPY . .

# Compila la aplicación en modo release
RUN cargo build --release

# Usa una imagen base más ligera para ejecutar la aplicación
FROM alpine:latest

# Copia el binario compilado desde el contenedor de compilación
COPY --from=builder /app/target/release/movieroom-backend /usr/local/bin/movieroom-backend

# Expone el puerto de la aplicación
EXPOSE 8080

# Comando para ejecutar la aplicación
CMD ["movieroom-backend"]