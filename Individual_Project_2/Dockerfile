FROM rust:latest as builder
ENV APP hugoweather
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["hugoweather"]
