use std::time::Duration;

use if_addrs::IfAddr;
use reqwest::Client;
use serde_json::Value as JsonValue;

/// 获取所有IP信息（本地IP、公网IPv4、公网IPv6）
pub async fn get_all_ips() -> Result<(String, String, String), Box<dyn std::error::Error>> {
    // 调用异步函数时需要用 await
    let local_ip = get_local_ip()?.unwrap_or_else(|| "未找到本地IPv4地址".to_string());
    let public_ipv4 = get_public_ipv4()
        .await
        .unwrap_or_else(|_| "获取公网IPv4失败".to_string());
    let public_ipv6 = get_public_ipv6()
        .await
        .unwrap_or_else(|_| "获取公网IPv6失败".to_string());

    Ok((local_ip, public_ipv4, public_ipv6))
}

/// 获取本机非回环IPv4地址（优先级：有线 > 无线 > 其他）
pub fn get_local_ip() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let if_addrs = if_addrs::get_if_addrs()?;
    let mut ethernet_ip: Option<String> = None;
    let mut wifi_ip: Option<String> = None;
    let mut other_ip: Option<String> = None;

    for if_addr in if_addrs {
        // 跳过回环接口和未启用的接口
        if if_addr.is_loopback() || !if_addr.is_oper_up() {
            continue;
        }

        // 只处理IPv4地址
        let ip_addr = match if_addr.addr {
            IfAddr::V4(v4) => v4.ip,
            IfAddr::V6(_) => continue,
        };

        let ip_str = ip_addr.to_string();

        // 过滤虚拟IP
        if is_virtual_ip(&ip_str) {
            continue;
        }

        // 根据接口类型分类
        match get_interface_type(&if_addr.name) {
            InterfaceType::Ethernet => {
                if ethernet_ip.is_none() {
                    ethernet_ip = Some(ip_str);
                }
            }
            InterfaceType::Wifi => {
                if wifi_ip.is_none() {
                    wifi_ip = Some(ip_str);
                }
            }
            InterfaceType::Other => {
                if other_ip.is_none() {
                    other_ip = Some(ip_str);
                }
            }
        }
    }

    // 按优先级返回
    Ok(ethernet_ip.or(wifi_ip).or(other_ip))
}

/// 获取公网IPv4地址（异步版本）
pub async fn get_public_ipv4() -> Result<String, Box<dyn std::error::Error>> {
    let urls = [
        "http://ipinfo.io/ip",
        "https://api.ipify.org",
        "https://ident.me",
        "http://myexternalip.com/raw",
    ];

    // 创建异步客户端（无 blocking 特性）
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    for url in urls {
        // 异步发送请求：用 await 等待结果
        match client.get(url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    // 异步读取响应文本：用 await
                    let ip = resp.text().await?.trim().to_string();
                    return Ok(ip);
                }
            }
            Err(e) => eprintln!("获取公网IP失败（{}）: {}", url, e),
        }
    }

    Err("所有公网IPv4获取服务均失败".into())
}

/// 获取公网IPv6地址（异步版本）
pub async fn get_public_ipv6() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    // 异步发送请求并等待响应
    let resp = client
        .get("https://api64.ipify.org?format=json")
        .send()
        .await?;

    if resp.status().is_success() {
        // 异步解析JSON响应：用 await
        let json: JsonValue = resp.json().await?;
        if let Some(ip) = json["ip"].as_str() {
            return Ok(ip.to_string());
        }
    }

    Err("获取公网IPv6失败".into())
}

/// 接口类型枚举
enum InterfaceType {
    Ethernet,
    Wifi,
    Other,
}

/// 判断接口类型（有线/无线/其他）
fn get_interface_type(name: &str) -> InterfaceType {
    let name_lower = name.to_lowercase();

    // 匹配有线接口
    if name_lower.contains("ethernet")
        || name_lower.contains("eth")
        || name_lower.contains("本地连接")
        || name_lower.contains("lan")
        || name_lower.starts_with("en")
    {
        return InterfaceType::Ethernet;
    }

    // 匹配无线接口
    if name_lower.contains("wi-fi")
        || name_lower.contains("wifi")
        || name_lower.contains("wireless")
        || name_lower.contains("wlan")
        || name_lower.contains("无线网络")
        || name_lower.starts_with("wl")
    {
        return InterfaceType::Wifi;
    }

    InterfaceType::Other
}

/// 判断是否为虚拟IP
fn is_virtual_ip(ip: &str) -> bool {
    let virtual_prefixes = [
        "169.254.", // 链路本地地址 (APIPA)
        "192.168.100.",
        "198.18.0.",
        "192.168.122.",    // libvirt
        "172.17.",         // Docker默认
        "172.18.",         // Docker网络
        "172.19.",         // Docker网络
        "172.20.",         // Docker网络
        "10.0.",           // 常见虚拟网络
        "0.0.0.0",         // 无效地址
        "255.255.255.255", // 广播地址
    ];

    virtual_prefixes.iter().any(|prefix| ip.starts_with(prefix))
}

// 测试示例（异步测试需要用 tokio::test 宏）
#[cfg(test)]
mod tests {
    use super::*;

    // 本地IP获取是同步的，普通测试即可
    #[test]
    fn test_get_local_ip() {
        if let Ok(Some(ip)) = get_local_ip() {
            println!("本地IPv4地址: {}", ip);
        } else {
            println!("未找到有效的本地IPv4地址");
        }
    }

    // 公网IP获取是异步的，需要用 tokio 测试运行时
    #[tokio::test]
    async fn test_get_public_ipv4() {
        if let Ok(ip) = get_public_ipv4().await {
            println!("公网IPv4地址: {}", ip);
        } else {
            println!("获取公网IPv4地址失败");
        }
    }

    #[tokio::test]
    async fn test_get_public_ipv6() {
        if let Ok(ip) = get_public_ipv6().await {
            println!("公网IPv6地址: {}", ip);
        } else {
            println!("获取公网IPv6地址失败");
        }
    }

    #[tokio::test]
    async fn test_get_all_ips() {
        if let Ok((local, ipv4, ipv6)) = get_all_ips().await {
            println!("本地IP: {}\n公网IPv4: {}\n公网IPv6: {}", local, ipv4, ipv6);
        } else {
            println!("获取IP信息失败");
        }
    }
}
