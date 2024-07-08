import http from 'k6/http';
import { check, sleep } from 'k6';
import { open } from 'k6'; // Ensure this import is correct

export let options = {
    stages: [
        { duration: '30s', target: 10 }, // Ramp up to 10 users over 30 seconds
        { duration: '1m', target: 10 },  // Stay at 10 users for 1 minute
        { duration: '30s', target: 0 },  // Ramp down to 0 users over 30 seconds
    ],
};

// Attempt to load the payload file
const payloadFilePath = 'payload.json';
let payload;

try {
    const fileContent = open(payloadFilePath);
    if (!fileContent) {
        throw new Error('File content is undefined or empty');
    }
    console.log(`File content: ${fileContent}`); // Debugging statement
    payload = JSON.parse(fileContent);
    console.log(`Parsed payload: ${JSON.stringify(payload)}`); // Debugging statement
} catch (error) {
    console.error(`Error reading or parsing the file at ${payloadFilePath}: ${error.message}`);
}

export default function () {
    if (!payload) {
        console.error('Payload is undefined. Exiting.');
        return;
    }

    let res = http.post('http://localhost:8080/promptpay/qrcode', JSON.stringify(payload), {
        headers: { 'Content-Type': 'application/json' },
    });

    check(res, { 'status was 200': (r) => r.status == 200 });
    sleep(1);
}
