// Import the required AWS SDK clients
const AWS = require('aws-sdk');
const dynamoDB = new AWS.DynamoDB.DocumentClient();

// Define the name of the DynamoDB table
const tableName = "lambda-dynamodb-stream";

exports.handler = async (event, context) => {
  try {
    // Define the item to be put into the table
    const item = {
      id: 'TEST ' + Math.floor(Math.random() * 1000)
    };

    // Define the DynamoDB put parameters
    const params = {
      TableName: tableName,
      Item: item
    };

    // Put the item into the DynamoDB table
    await dynamoDB.put(params).promise();

    // Return a successful response
    return {
      statusCode: 200,
      body: JSON.stringify({ message: 'Item added to the table successfully.' })
    };
  } catch (error) {
    console.error(`Error putting item into table: ${error.message}`);
    return {
      statusCode: 500,
      body: JSON.stringify({ message: 'Error putting item into table.' })
    };
  }
};

