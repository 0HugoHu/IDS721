console.log('Loading function');

const AWS = require('aws-sdk');
const s3 = new AWS.S3();

exports.handler = async (event, context) => {
	console.log(JSON.stringify(event, null, 2));
    event.Records.forEach(function(record) {
        console.log(record.eventID);
        console.log(record.eventName);
        console.log('DynamoDB Record: %j', record.dynamodb);
    });
	  
	    
  try {
    // Set the bucket name and an object key using the current timestamp
    const bucketName = 'dynamo-bucket-s3'; // Replace with your S3 bucket name
    const objectKey = `request-${Date.now()}.json`;

    // Prepare the S3 putObject parameters
    const params = {
      Bucket: bucketName,
      Key: objectKey,
      Body: JSON.stringify(event),
      ContentType: 'application/json',
    };

    // Upload the JSON event data to S3
    await s3.putObject(params).promise();

    // Return a successful response
    return {
      statusCode: 200,
      body: JSON.stringify({
        message: 'Request saved to S3 successfully.',
        objectKey: objectKey,
      }),
    };
  } catch (error) {
    // Log the error and return an error response
    console.error('Error saving request to S3:', error);
    return {
      statusCode: 500,
      body: JSON.stringify({
        message: 'An error occurred while saving the request to S3.',
        error: error.message,
      }),
    };
  }
};

