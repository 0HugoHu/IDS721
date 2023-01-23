# Introduction

In this Demo, user gives a floating number of altitude in meters, the web returns the city name, its country and location in the world closeset to that altitude.

# Feature
- **Use "actix-web" for Http Response**
- **Deployed on AWS EC2 Cloud Server, and Visitable through Public IPV4 Address**
- **Written in Rust (which is a barnd-new language for me)**

# Usage
The format is: http://3.232.64.14/alt/ALTITUDE

Replace **"ALTITUDE"** with a floating number in the range of **f32** in Rust

Enter the full url in your browser.

<br />
For example:

http://3.232.64.14/alt/1789.32
![](https://s1.ax1x.com/2023/01/24/pSYt7LD.png)
http://3.232.64.14/alt/-1000
![](https://s1.ax1x.com/2023/01/24/pSYtbee.png)
