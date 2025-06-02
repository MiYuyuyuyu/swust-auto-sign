# 西南科技大学自动签到程序

这是一个用于西南科技大学每日在校打卡的自动签到程序，通过模拟浏览器请求实现一键签到功能。

## 功能特点

- ✅ 完全自动化签到流程  
- 🔒 通过环境变量保护敏感信息  
- 📅 自动处理日期格式  
- 📊 详细的日志输出  
- ⏱️ 签到完成后自动退出  

## 使用说明

### 环境要求

1. 安装 [Rust](https://www.rust-lang.org/tools/install) 编程语言环境  
2. 安装 Cargo（Rust 的包管理器）  

### 配置环境变量

在运行程序前，需要设置以下环境变量：

```bash
# 用户信息
export ID="你的学号"
export PASSWORD="你的密码"
export NAME="你的姓名"

# 任务信息
export TASK_ID="任务ID"

# 位置信息
export LOCATION_IDS="位置ID1,位置ID2,位置ID3,位置ID4"
export SIGN_COORDINATE="经度,纬度"  # 例如：104.697718,31.536998
export SIGN_LOCATION_NAME="位置名称"

# 班级和教师信息
export LABS="你的班级"
export S_TEACHER="教师姓名_教师工号"
export S_TUTOR_TEACHER="导师姓名_导师工号"
```
顺便一提是这个网站的密码(不是一站式的密码)
[![页面截图图片](assets\img\页面截图.png "演示图片")](https://github.com/MiYuyuyuyu/swust-auto-sign/blob/master/asset/img/%E9%A1%B5%E9%9D%A2%E6%88%AA%E5%9B%BE.png)

## 运行程序

```
# 克隆仓库
git clone https://github.com/MiYuyuyuyu/swust-auto-sign.git
cd swust-auto-sign

# 运行程序
cargo run
```

## 环境变量说明

| 变量名 | 描述 | 示例值 |
|--------|------|--------|
| `ID` | 学号 | `51202XXXXX` |
| `PASSWORD` | 密码 | `XXXXXXXX` |
| `NAME` | 姓名 | `XXX` |
| `TASK_ID` | 任务ID | `b1f3d078904adb0dbdc` |
| `LOCATION_IDS` | 位置ID列表（逗号分隔） | `5bca4e88e26bcdbc384,f72a2ffa6b4b038d4c9,ee28e12a5e0a8bc817b,9955c159178b709741b` |
| `SIGN_COORDINATE` | 签到坐标（经度,纬度） | `104.697718,31.536998` |
| `SIGN_LOCATION_NAME` | 签到位置名称 | `四川省绵阳市涪城区青义镇青蒿路西南科技大学青义校区` |
| `LABS` | 班级 | `软件2XXX` |
| `S_TEACHER` | 教师信息 | `张X_XXXXXXXXXX` |
| `S_TUTOR_TEACHER` | 导师信息 | `XX_XXXXXXXX` |

## 如何获取所需信息

1. **任务ID**：在签到页面的URL中找到 `task_id` 参数  

在签到页面的URL中找到 `task_id` 参数：
https://jkxb.swust.edu.cn/#/signByTaskMap?task_id=b1f3d078904adb0dbdc
└─────────────────────────────────────────────┘


2. **位置ID**：
使用以下油猴脚本捕获位置ID请求：

```javascript
// ==UserScript==
// @name         Capture Sign Requests
// @namespace    http://tampermonkey.net/
// @version      1.0
// @description  捕获签到请求中的位置ID
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

        // 捕获位置ID请求
        if (args[0].includes('/get_multiple_location_path')) {
            console.log('捕获到位置ID请求:');
            console.log('URL:', args[0]);
            console.log('Body:', args[1]?.body);
            
            // 解析请求体获取位置ID
            const body = JSON.parse(args[1]?.body);
            if (body.location_ids) {
                console.log('位置ID列表:', body.location_ids.join(','));
                alert(`捕获到位置ID: ${body.location_ids.join(',')}`);
            }
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

        get responseText() {
            const text = super.responseText;
            if (this._url.includes('/get_multiple_location_path')) {
                // 解析请求数据获取位置ID
                try {
                    const data = JSON.parse(this._data);
                    if (data.location_ids) {
                        console.log('捕获到位置ID列表:', data.location_ids.join(','));
                        alert(`捕获到位置ID: ${data.location_ids.join(',')}`);
                    }
                } catch (e) {
                    console.log('解析位置ID请求失败:', e);
                }
            }
            return text;
        }
    };
})();
```
使用步骤：
安装 Tampermonkey 浏览器扩展
创建新脚本并粘贴以上代码
访问签到页面
页面加载时会自动捕获位置ID并弹出提示
[![油猴演示图片](assets\img\演示油猴.png "演示图片")](https://github.com/MiYuyuyuyu/swust-auto-sign/blob/master/asset/img/%E6%BC%94%E7%A4%BA%E6%B2%B9%E7%8C%B4.png)


3. **签到坐标**：使用高德地图获取实际位置的经纬度  
4. **教师信息**：如上图  

## 注意事项

1. 请确保环境变量中的位置ID列表用逗号分隔，不能有空格  
2. 坐标格式必须是`经度,纬度`，小数点位数不限  
3. 教师信息格式必须是`姓名_工号`  
4. 程序会在签到后等待5秒退出，方便查看结果  
5. 请勿将包含敏感信息的代码或配置文件上传到公共仓库  

## 贡献指南

欢迎提交 Pull Request 改进本项目：

## 许可证

本项目采用 [MIT 许可证](LICENSE)

## 免责声明

本项目仅用于学习和技术交流目的，请遵守学校相关规定，合理使用程序。因使用本项目而产生的任何问题，作者概不负责。