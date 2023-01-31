# Week 1 Demo
## Introduction

In this Demo, user gives a floating number of altitude in meters, the web returns the city name, its country and location in the world closeset to that altitude.

## Feature
- **Use "actix-web" for Http Response**
- **Deployed on AWS EC2 Cloud Server, and Visitable through Public IPV4 Address**
- **Written in Rust (which is a brand-new language for me)**

## Usage
The format is: http://3.232.64.14/alt/ALTITUDE

Replace **"ALTITUDE"** with a floating number in the range of **f32** in Rust

Enter the full url in your browser.

<br />
For example:

http://3.232.64.14/alt/1789.32
![](https://s1.ax1x.com/2023/01/24/pSYt7LD.png)
http://3.232.64.14/alt/-1000
![](https://s1.ax1x.com/2023/01/24/pSYtbee.png)


# Week 2 Demo
## Introduction

In this Demo, I learned how to use Amazon Lambda to deploy a cloud function. The demo returns the maximum frequency of any char in the given string.

## Feature
- **Learn how to write an Amazon Lambda in Rust.**
- **Deployed on Amazon Lambda.**

## Usage
> git clone https://github.com/0HugoHu/IDS721.git
> 
> cargo build --release
> 
> cp target/lambda/release/marco-polo-lambda ./bootstrap
> 
> zip lambda.zip bootstrap
> 
> // Upload this file to your Amazon Lambda
> 

![](/images/demo2.png)

#

# Individual Project 1 Demo

## Introduction

This project aims to provide a CLI tool in data engineering, specifically, calcualte the statistics from worldwide ciry altitudes. 

## Usage
> git clone https://github.com/0HugoHu/IDS721.git
> 
> nohup cargo run all // enable background process
> 

The formats are:

**http://3.232.64.14/alt/$ALTITUDE**

where, $ALTITUDE = **numerical numbers** (e.g., 172.2, -25)

OR

**http://3.232.64.14/sta/$VARIABLE**

where, $VARIABLE = **mean, median, range, stddev, variance**.

![](/images/indi1.png)

![](/images/indi2.png)

![](/images/indi3.png)



**More features will be added...**

