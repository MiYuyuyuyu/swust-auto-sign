// ==UserScript==
// @name         Capture Sign Requests
// @namespace    http://tampermonkey.net/
// @version      1.0
// @description  捕获签到请求
// @match        https://jkxb.swust.edu.cn/*
// @grant        none
// ==/UserScript==

(function() {
    'use strict';

    // 保存原始fetch方法
    const originalFetch = window.fetch;

    // 重写fetch方法
    window.fetch = async function(...args) {
        const response = await originalFetch.apply(this, args);

        // 检查是否是签到请求
        if (args[0].includes('/sign_edit')) {
            console.log('捕获到签到请求:');
            console.log('URL:', args[0]);
            console.log('Method:', args[1]?.method || 'GET');
            console.log('Headers:', args[1]?.headers);
            console.log('Body:', args[1]?.body);

            // 克隆响应以便读取
            const clone = response.clone();
            clone.json().then(data => {
                console.log('Response:', data);
            }).catch(err => {
                console.log('Response text:', clone.text());
            });
        }

        return response;
    };

    // 保存原始XMLHttpRequest
    const originalXHR = window.XMLHttpRequest;

    // 重写XMLHttpRequest
    window.XMLHttpRequest = class extends originalXHR {
        open(method, url) {
            this._method = method;
            this._url = url;
            super.open(method, url);
        }

        send(data) {
            this._data = data;
            super.send(data);
        }

        setRequestHeader(header, value) {
            this._headers = this._headers || {};
            this._headers[header] = value;
            super.setRequestHeader(header, value);
        }

        get responseText() {
            const text = super.responseText;
            if (this._url.includes('/sign_edit')) {
                console.log('捕获到签到请求(XHR):');
                console.log('URL:', this._url);
                console.log('Method:', this._method);
                console.log('Headers:', this._headers);
                console.log('Body:', this._data);
                console.log('Response:', text);
            }
            return text;
        }
    };
})();