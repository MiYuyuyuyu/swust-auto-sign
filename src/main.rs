use reqwest::Client;
use chrono::{Local, Utc};
use serde_json::json;
use anyhow::{Result, Context};
use uuid::Uuid;
use std::env;

#[derive(Debug)]
struct Config {
    id: String,
    password: String,
    name: String,
    task_id: String,
    location_ids: Vec<String>,
    sign_coordinate: [f64; 2],
    sign_location_name: String,
    labs: String,
    s_teacher: String,
    s_tutor_teacher: String,
}

impl Config {
    fn from_env() -> Result<Self> {
        Ok(Config {
            id: env::var("ID").context("缺少环境变量 ID")?,
            password: env::var("PASSWORD").context("缺少环境变量 PASSWORD")?,
            name: env::var("NAME").context("缺少环境变量 NAME")?,
            task_id: env::var("TASK_ID").context("缺少环境变量 TASK_ID")?,
            location_ids: env::var("LOCATION_IDS")
                .context("缺少环境变量 LOCATION_IDS")?
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            sign_coordinate: {
                let coords = env::var("SIGN_COORDINATE")
                    .context("缺少环境变量 SIGN_COORDINATE")?
                    .split(',')
                    .map(|s| s.trim().parse::<f64>())
                    .collect::<Result<Vec<f64>, _>>()
                    .context("无效的坐标格式")?;
                
                if coords.len() != 2 {
                    return Err(anyhow::anyhow!("坐标格式错误，应为经度,纬度"));
                }
                
                [coords[0], coords[1]]
            },
            sign_location_name: env::var("SIGN_LOCATION_NAME")
                .context("缺少环境变量 SIGN_LOCATION_NAME")?,
            labs: env::var("LABS").context("缺少环境变量 LABS")?,
            s_teacher: env::var("S_TEACHER").context("缺少环境变量 S_TEACHER")?,
            s_tutor_teacher: env::var("S_TUTOR_TEACHER")
                .context("缺少环境变量 S_TUTOR_TEACHER")?,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 从环境变量加载配置
    let config = Config::from_env()?;
    println!("✅ 配置加载成功: {:?}", config);

    // 创建带cookie存储的HTTP客户端
    let client = Client::builder()
        .cookie_store(true)
        .build()
        .context("创建HTTP客户端失败")?;

    // 1. 登录请求
    let login_url = "https://jkxb.swust.edu.cn/user/login";
    let login_data = json!({
        "ID": config.id,
        "password": config.password,
        "loginByScanCode": ""
    });

    println!("正在登录...");
    let login_res = client
        .post(login_url)
        .json(&login_data)
        .send()
        .await
        .context("登录请求失败")?;

    // 检查登录结果
    if !login_res.status().is_success() {
        return Err(anyhow::anyhow!("登录失败: {}", login_res.status()));
    }
    
    // 2. 准备签到数据
    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let task_process_id = format!("{} 18:00:00", current_date);
    
    // 使用UUID生成sign_id
    let sign_id = Uuid::new_v4().to_string();
    
    // 构建introduction内容
    let introduction = json!({
        "签到方式": "定位签到",
        "签到内容": {
            "签到状态": "签到成功",
            "使用位置信息编号": config.location_ids,
            "使用范围信息": "[[104.698088,31.538095],[104.698116,31.536891],[104.699554,31.536843],[104.699639,31.53745],[104.69956,31.538068]],[[104.697229,31.538091],[104.697225,31.536808],[104.698706,31.536889],[104.698714,31.538104]],[[104.691899,31.538442],[104.691451,31.537912],[104.691344,31.537348],[104.692351,31.53719],[104.692501,31.537583],[104.692716,31.538026]],[[104.693312,31.538576],[104.693325,31.538334],[104.693662,31.538325],[104.693957,31.53799],[104.694247,31.538142],[104.693854,31.538617],[104.693681,31.538608]]",
            "签到坐标": config.sign_coordinate,
            "签到位置名称": config.sign_location_name
        }
    }).to_string();
    
    // 当前时间（ISO格式）精确到毫秒
    let create_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    
    // 完整的签到请求体
    let sign_data = json!([
        {
            "sign_id": sign_id,
            "ID": config.id,
            "name": config.name,
            "task_id": config.task_id,
            "task_process_id": task_process_id,
            "task_name": "23级软件-2025年春季学期每日在校打卡",
            "sign_status": "1",
            "introduction": introduction,
            "labs": config.labs,
            "S_Teacher": config.s_teacher,
            "S_tutor_Teacher": config.s_tutor_teacher,
            "create_time": create_time
        }
    ]);

    println!("签到数据: {}", serde_json::to_string_pretty(&sign_data)?);
    
    // 3. 发送签到请求
    let sign_url = "https://jkxb.swust.edu.cn/task/sign_edit";
    println!("正在签到...");
    let sign_res = client
        .post(sign_url)
        .json(&sign_data)
        .send()
        .await
        .context("签到请求失败")?;

    // 4. 处理签到结果
    let sign_status_code = sign_res.status();
    let sign_response = sign_res
        .text()
        .await
        .context("获取签到响应失败")?;
    
    println!("签到状态码: {}", sign_status_code);
    println!("签到响应: {}", sign_response);
    
    if sign_status_code.is_success() {
        println!("✅ 签到成功!");
    } else {
        println!("❌ 签到失败!");
    }

    println!("5s后退出\n祝你生活愉快!");
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}