# Individual Project 4

## Introduction

In this project, I learn how to use serverless service. I plan to build an end-to-end pipeline as below:

<img src="../images/indi4 (3).png" width="50%">

## Week 9

The first week, I learned AWS Lambda by created a javascript function to return the status of a reponse.

<img src="../images/indi4 (2).png"  width="75%">

Then I mannually test this function by created an ```input.txt``` to mock the action stream going out from DynamoDB:

```txt
{
   "Records":[
      {
         "eventID":"1",
         "eventName":"INSERT",
         "eventVersion":"1.0",
         "eventSource":"aws:dynamodb",
         "awsRegion":"us-east-1",
         "dynamodb":{
            "Keys":{
               "Id":{
                  "N":"101"
               }
            },
            "NewImage":{
               "Message":{
                  "S":"New item!"
               },
               "Id":{
                  "N":"101"
               }
            },
            "SequenceNumber":"111",
            "SizeBytes":26,
            "StreamViewType":"NEW_AND_OLD_IMAGES"
         },
         "eventSourceARN":"stream-ARN"
      },
      ...
   ]
}
```

Finally I called this lambda function in the AWS Shell, the output returns a status code of 200, which indicates correction.

![](../images/indi4%20(1).png)

