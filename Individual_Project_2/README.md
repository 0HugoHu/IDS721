## Introduction
In project 2, I deployed this app on AWS AppRunner and Kubernetes (Minikube for local) for Rust-based microservice.

This app parses user's input of a city name, and returns the real-time weather using OpenWeather API. 

Now, it also achieves history 365-day AQI reports.

### 1. Containerized Actix Microservice

**Change** docker file if you need:

```dockerfile
FROM rust:latest as builder
ENV APP hugoweather
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["hugoweather"]
```

### 2. Configure OpenWeather API

Though I keeped my api_key in the code, please **DO NOT USE IT!**

Register your account, and get your own **FREE** api_key [here](https://openweathermap.org/).

**Replace** "url2" in lib.rs with your own's:
```rust
// Get response from weather api
async fn get_response(location: &str) -> JsonValue {
    let url1 = "http://api.openweathermap.org/data/2.5/weather?q=";
    let url2 = "&appid=1f565f89dbe934dc5916a2cc3f943d76"; // Replace this
    let url = [url1, location, url2].concat();
    let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();
    json::parse(&resp).unwrap()
}
```

### 3. Local Deployment
**Build** docker image and run it:

```bash
docker build -t hugoweather . 
docker run -it --rm -p 8080:8080 hugoweather
```

**Open** your broswer and visit: http://127.0.0.1:8080/

You are expected to see this welcome page:

![](../images/indi2_%20(2).png)


</br>

## Method 1.1 Push to AWS ECR
### 1. Prerequisites

- **Docker is properly installed and running**
- **You have created an AWS account**

### 2. Configure ECR

**Create** a new blank ECR image in your AWS dashboard: [AWS ECR](https://us-east-1.console.aws.amazon.com/ecr)

**Create** a new access key in your AWS IAM dashboard: [AWS IAM](https://us-east-1.console.aws.amazon.com/iamv2)

You need also **grant sufficient permissions** to the key. See this article if you meet any problems: [Help](https://www.freecodecamp.org/news/build-and-push-docker-images-to-aws-ecr/)

**Install** AWS CLI:
```bash
sudo apt-get install awscli
```

**Configure** your local credentials:
```bash
aws configure
# enter your access key and secrets, select default region: us-east-1
```

**Retrieve** an authentication token and authenticate your Docker client to your registry.
```bash
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin YOUR_ECR_ID.dkr.ecr.us-east-1.amazonaws.com
# replace YOUR_ECR_ID with your image url
```

**Tag** your image so you can push the image to this repository.
```bash
docker tag hugoweather:latest YOUR_ECR_ID.dkr.ecr.us-east-1.amazonaws.com/hugoweather:latest
# replace YOUR_ECR_ID with your image url
```

**Run** the following command to push this image to your newly created AWS repository:
```bash
docker push YOUR_ECR_ID.dkr.ecr.us-east-1.amazonaws.com/hugoweather:latest
# replace YOUR_ECR_ID with your image url
```

![](../images/indi2_%20(1).png)

</br>

## Method 1.2 Publish to AWS App Runner

**Create** a new App Runner service: [AWS APP Runner](https://us-east-1.console.aws.amazon.com/apprunner) 

**Select** the image you built, and choose **Auto Deploy**.

**Wait** until health check is automatically completed.

![](../images/indi2_%20(3).png)

</br>

## Method 1.3 Demo

**Try demos with AWS App Runner Default Domain: https://erp7imdtkd.us-east-1.awsapprunner.com/**

**Your broswer will automatically convert whitespace into "%20"**

|**Query** real-time weather by city name|
|---|
|![](../images/indi2_%20(4).png)|
|![](../images/indi2_%20(5).png)|

|**Query** historical 365-day AQI by city name|
|---|
|![](../images/indi2_%20(6).png)|
|![](../images/indi2_%20(7).png)|

</br>

## Method 2.1 Push to DockerHub
### 1. Prerequisites

**Log** in to your DockerHub by:
```bash
docker login
```

### 2. Push docker binary

```bash
docker build . -t $USERNAME/$APP-NAME
docker push $USERNAME/$APP-NAME
# Replace $USERNAME with your DockerHub username, and $APP-NAME with you app name
```
</br>

## Method 2.2 Run MiniKube (Local Kubernetes)

**Start** your cluster by:
```bash
minikube start
```

Then **create** a deployment: 
```bash
kubectl create deployment $NAME --image=registry.hub.docker.com/$USERNAME/$APP-NAME
# Replace $NAME with any cluster name
```

**Make** sure your deployment is set up:
```bash
kubectl get deployments
```

**Create** service and expose port: 
```bash
kubectl expose deployment $NAME --type=LoadBalancer --port=8080
```

**View** service status:
```bash
kubectl get service $NAME
```

**Run** service:
```bash
minikube service $NAME  --url
```

Now, **copy** the url from the output, and copy it into your web browser.
![](../images/indi2_%20(8).png)

You successfully deployed your app to the Minikube cluster.
![](../images/indi2_%20(9).png)

</br>

## Method 2.3 Cleanup Build Files
```bash
kubectl delete service $NAME
kubectl delete deployment $NAME
minikube stop
```
![](../images/indi2_%20(10).png)

</br>

## Acknoledgement
**Special thanks to [@Jiaxin Ying](https://github.com/helenyjx) who's well-illustrated [documentation](https://github.com/nogibjj/Jiaxin-P2-Rust-Minikube) helped me a lot.**
