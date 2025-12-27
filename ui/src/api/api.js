import request from '@/utils/request'

// 获取全部列表
export function get_all() {
  return request({
    url: '/getall',
    method: 'get',
  })
}

// 更新设置
export function set_setting(data) {
  return request({
    url: '/api/setting',
    method: 'post',
    data,
  })
}

// 获取设置
export function get_setting() {
  return request({
    url: '/api/setting',
    method: 'get',
  })
}

// 获取打印机列表以及状态
export function getprinter() {
  return request({
    url: '/api/printer',
    method: 'get',
  })
}

// 上传文件
export function upload(data) {
  return request({
    url: '/api/upload',
    method: 'post',
    data
  })
}

// 打印文件
export function print(data) {
  return request({
    url: '/print',
    method: 'post',
    data
  })
}