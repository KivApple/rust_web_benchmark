docker run --name axum -d -p 127.0.0.1:3000:3000 rust_web_benchmark_axum:latest
docker run --name actix -d -p 127.0.0.1:3001:3000 rust_web_benchmark_actix:latest
docker run --name rocket -d -p 127.0.0.1:3002:3000 rust_web_benchmark_rocket:latest

target\release\bench http://localhost:3001/test/simple 100 100
target\release\bench http://localhost:3000/test/simple 100 100
target\release\bench http://localhost:3002/test/simple 100 100

docker stop axum
docker stop actix
docker stop rocket
docker rm axum
docker rm actix
docker rm rocket

docker run --name axum -d -p 127.0.0.1:3000:3000 rust_web_benchmark_axum:latest
docker run --name actix -d -p 127.0.0.1:3001:3000 rust_web_benchmark_actix:latest
docker run --name rocket -d -p 127.0.0.1:3002:3000 rust_web_benchmark_rocket:latest

target\release\bench http://localhost:3001/test/timed 100 100
target\release\bench http://localhost:3000/test/timed 100 100
target\release\bench http://localhost:3002/test/timed 100 100

docker stop axum
docker stop actix
docker stop rocket
docker rm axum
docker rm actix
docker rm rocket

docker run --name axum -d -p 127.0.0.1:3000:3000 rust_web_benchmark_axum:latest
docker run --name actix -d -p 127.0.0.1:3001:3000 rust_web_benchmark_actix:latest
docker run --name rocket -d -p 127.0.0.1:3002:3000 rust_web_benchmark_rocket:latest

target\release\bench http://localhost:3001/test/bcrypt 10 50
target\release\bench http://localhost:3000/test/bcrypt 10 50
target\release\bench http://localhost:3002/test/bcrypt 10 50

docker stop axum
docker stop actix
docker stop rocket
docker rm axum
docker rm actix
docker rm rocket

@pause
