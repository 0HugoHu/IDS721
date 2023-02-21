## Week 5 Demo

In this week, I investigated the outstanding paralielism feature in Rust. This project also compares md5 and sha256 algorithms performance by benchmarking the checksum of several files.

## Usage

```sh
git clone https://github.com/0HugoHu/IDS721.git

cd Week5

time cargo --quiet run -- $SUB-COMMAND

# Replace $SUB-COMMAND with: parallel-sha256, parallel-md5, serial-sha256, serial-md5
```

## Result

|Checksum Algm|Time(s)|
|---|---|
|Serial-MD5|0.430 |
|Serial-SHA256|0.375|
|Parallel-MD5|0.189|
|Parallel-SHA256|0.174|

According to [Jeff M Lowery](https://www.freecodecamp.org/news/md5-vs-sha-1-vs-sha-2-which-is-the-most-secure-encryption-hash-and-how-to-check-them/#:~:text=The%20SHA%2D256%20algorithm%20returns,MD5%20or%20SHA%2D1%20hashes.), a SHA-256 hash is about **20-30% slower** to calculate than either MD5 or SHA-1 hashes.
The results show that, however, **MD5**'s calculation time is a little bit longer. This may due to the relatively small file size.

But when applying parallel calculation, the time is saved a lot!


![](images/demo5%20(1).png)
![](images/demo5%20(2).png)
![](images/demo5%20(3).png)
![](images/demo5%20(4).png)
