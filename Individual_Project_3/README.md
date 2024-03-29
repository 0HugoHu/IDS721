## Introduction
In project 3, I plan to use **AWS S3** for large file storage, and deploy it on **Amazon SageMaker Studio** for end-to-end ML workflow.

**The first step (and for this week) is to try training ML model locally with Rust.**


### 1. Rust ML Example

#### 1.1. Linfa - A Meta-crate for ML

Linfa is a higher-level meta-crate that includes common helpers for data processing and algorithms for many areas of machine learning, including [[1]](https://blog.logrocket.com/machine-learning-in-rust-using-linfa/):

- Linear regression
- Data clustering
- Kernel methods
- Logistic regression
- Bayes
- SVMs

#### 1.2. Minimal ML Dataset

The first week's work is to build a simple machine learning application in Rust. A very small dataset containing only 100 records is used.

There are two datasets in ```data``` folder, one is for traning ```train.csv``` and the other is for testing ```test.csv```.

Both datasets look like this:

```
67.94685548	46.67857411	0
70.66150955	92.92713789	1
```

Where, the first and the second columns are the two ```features```, and the last column is the ```target```, and ```1``` is for acceptance, and ```0``` means denied.


#### 1.3. Train Model Locally

This examples uses ```logistic regression``` model to predict the outcome.

To train the model, run:

```sh
cd Individual_Project_3
cargo run all
```

The output is:

![](../images/indi3_%20(1).png)

</br>

### 2. Use AWS S3

#### 2.1. AWS S3 Creation

The S3 bucket can be created directly from console.

![](../images/indi3_%20(2).png)

### 2.2. Import file

By clicking the upload button, files can be uploaded.

![](../images/indi3_%20(3).png)

### 3. SageMaker Deploy

In the field of data science, the SageMaker preferred a Jupyter Notebook-style launcher, and they only support Python or R. So I switched to Python3 for the training and visualization.

The whole process consists of five steps:

- **Importing Modules**
- **Data Preprocessing**
- **Exploratory Data Analysis**
- **Data Cleaning**
- **Model Training**

</br>

For the full result, please refer to this **[REPORT](Multiclass-Resume-Classifier.ipynb)**

![](../images/indi3_%20(4).png)

<ins>
I figured out that Amazon SageMaker even with <b>4vCPU and 64GB-RAM</b> is so slow in data processing step compared to my own laptop. So I switched to my local Jupyter Notebook for the following training.
</ins>


