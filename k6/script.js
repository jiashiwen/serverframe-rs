import http from 'k6/http';
import {sleep} from 'k6';

export default function () {
    const url = 'http://127.0.0.1:3000/api/tpost';
    const payload = JSON.stringify({"username": "abc"});

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    http.post(url, payload, params);
    // sleep(0.01);

}
