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

<br />

# Week 3 Demo
## Introduction

In this Demo, user gives the saturation value she wants to adjust, and the program generate a new image with expected saturation named 'rust_logo_new.png'

## Feature
- **Use "image" library for image processing**

## Usage
> git clone https://github.com/0HugoHu/IDS721.git
> 
> cd Week3
>
> cargo run all 64 // Any value between 0-255
> 

<br />
For example:


|Source Image|Output Image|
|---|---|
|![](/Week3/rust_logo.png)|![](/Week3/rust_logo_new.png)|


<br />

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

OR

**http://3.232.64.14/figure/$PLACEHOLDER**

where, $PLACEHOLDER = **Anything but empty**.

![](/images/indi4.png)

<br />

**More to say...**

In week 3's work, I planned to show the world's city's altitude with its frequency. I tried:

- **Draw a figure directly on the front-end.** But I didn't want anymore dependencies, and realized that I should focus on Rust and its middleware, so I changed my plan.
- **Send plain text as HttpResponse.** I even found a nice repository, named: textplots-rs (https://github.com/loony-bean/textplots-rs), which generates dot-based char figure from given expressions. But then I found some of these characters were in Unicode coding, and they could not be displayed properly on web page. Then I switched to:
- **Send picture instead of text.** However, I messed up my mind on how to do with Rust syntax. I found all code online not working for me.
- **Send html page including a img tag.** This worked as html could be return simply as content_type("text/html").body(html). But I didn't know the absolute path of my image, and I tried all possible ways but still failed.
- **Send html page including pure text-expression of the figure.** This way appearently worked, but the problem was again the Unicode coding. Finally I found the solution: adding "charset=UTF-8" into the content_type.

So, it looks like I did just a few work this week, but I did spend a lot of time on it (also fixed some bad data in database).

Example of using testplots-rs library:
![](/images/indi5.png)

<br />

