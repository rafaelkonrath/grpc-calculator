import grpc from 'k6/net/grpc';
import { check, sleep } from 'k6';

const client = new grpc.Client();
client.load(['proto'], 'calculator.proto');

export const options = {
  stages: [
    { target: 50, duration: '3m' },
    { target: 100, duration: '0m' },
    { target: 100, duration: '3m' },
    { target: 200, duration: '3m' },
  ]
}

export default () => {
  client.connect('0.0.0.0:50051', {
    plaintext: true
  });

  const data = { "a": 1012, "b": 1012 };
  const response = client.invoke('calculator.Calculator/Add', data);

  check(response, {
    'status is OK': (r) => r && r.status === grpc.StatusOK,
  });

  console.log(JSON.stringify(response.message));

  client.close();
  sleep(1);
};

