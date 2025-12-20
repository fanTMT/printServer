const formatDate = (dateInput) => {
    if (!dateInput) return '未知时间';
    try {
        let date;
        // 判断输入是数组还是字符串
        if (Array.isArray(dateInput)) {
            date = parseArrayDate(dateInput); // 数组转 Date
        } else {
            date = new Date(dateInput); // 字符串转 Date
        }

        // 验证日期是否有效
        if (isNaN(date.getTime())) {
            return '无效时间';
        }

        return date.toLocaleString('zh-CN', {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            hour12: false
        });
    } catch {
        return '未知时间';
    }
};

// 核心修复：适配 Rust OffsetDateTime::now_local() 序列化数组
const parseArrayDate = (dateArray) => {
    if (!Array.isArray(dateArray) || dateArray.length < 7) {
        return new Date('invalid');
    }
    // 正确映射 Rust OffsetDateTime 序列化字段
    const year = dateArray[0];          // 年（正确）
    const dayOfYear = dateArray[1];     // 一年中的第几天（1-366，后端正确返回）
    const hours = dateArray[2];         // 时（0-23，正确）
    const minutes = dateArray[3];       // 分（0-59，正确）
    const seconds = dateArray[4];       // 秒（0-59，正确）
    const nanoseconds = dateArray[5];   // 纳秒（0-999999999，需转毫秒）
    // dateArray[6] 是时区偏移分钟（8 → UTC+8，Rust已转本地时间，无需处理）
    // 步骤1：验证 dayOfYear 有效性（避免异常值）
    const isLeapYear = (year % 4 === 0 && year % 100 !== 0) || year % 400 === 0;
    const maxDays = isLeapYear ? 366 : 365;
    if (dayOfYear < 1 || dayOfYear > maxDays) {
        return new Date('invalid');
    }
    // 步骤2：根据年和 dayOfYear 计算 月/日（1-12月，1-31日）
    const [month, day] = calculateMonthDay(year, dayOfYear);
    // 步骤3：纳秒转毫秒（1纳秒 = 1e-6毫秒，取整避免小数）
    const milliseconds = Math.floor(nanoseconds / 1000000);
    // 步骤4：创建本地时间（Rust 已返回本地时间，直接用 Date 构造函数）
    const localDate = new Date(year, month - 1, day, hours, minutes, seconds, milliseconds);
    return localDate;
};

const calculateMonthDay = (year, dayOfYear) => {
    // 每个月的天数（平年）
    const monthDays = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    // 闰年2月加1天
    if ((year % 4 === 0 && year % 100 !== 0) || year % 400 === 0) {
        monthDays[1] = 29;
    }

    let remainingDays = dayOfYear;
    let month = 0; // 0 对应1月，最终返回需+1

    while (month < 12) {
        if (remainingDays <= monthDays[month]) {
            return [month + 1, remainingDays]; // 月份转1-12格式
        }
        remainingDays -= monthDays[month];
        month++;
    }
    return [12, remainingDays];
};

export default formatDate