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
    url: '/api/set_setting',
    method: 'post',
    data,
  })
}

// 获取设置
export function get_setting() {
  return request({
    url: '/api/get_setting',
    method: 'get',
  })
}
