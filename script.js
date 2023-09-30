import http from 'k6/http';

export const options = {
  stages: [
    { duration: '30s', target: 500 },
    { duration: '1m', target: 1000 },
    { duration: '30s', target: 500 }

  ],
};

export default function () {
  const url = 'http://127.0.0.1:3000';
  const payload = JSON.stringify({
    first_name: "Judd Misael",
    last_name: "Baguio",
    username: "juddbaguio",
    password: "12345678"
});

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  http.post(url, payload, params);
}
