<p align="center">
  <img src="../icon.png" width="160" alt="Qaren Logo">
</p>

<h1 align="center">Qaren (قارن)</h1>

<p align="center">
  <a href="../README.md">English</a> | 
  <a href="README.zh.md">中文</a> | 
  <a href="README.ru.md">Русский</a> | 
  <a href="README.ar.md">العربية</a> | 
  <a href="README.fa.md">فارسی</a> | 
  <a href="README.ja.md">日本語</a>
</p>

<p align="center">
  <b>نسل جدید ابزارهای مقایسه تنظیمات و نسخه‌های پشتیبان سیستم.</b><br>
  طراحی شده برای عصر مدرن DevOps: معنایی (Semantic)، امن، و فوق‌العاده سریع.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-1.0.0-orange.svg" alt="Version">
  <img src="https://img.shields.io/badge/PRs-welcome-cyan.svg" alt="PRs Welcome">
  <a href="https://github.com/qaren-cli/qaren/actions/workflows/release.yml">
    <img src="https://github.com/qaren-cli/qaren/actions/workflows/release.yml/badge.svg?branch=master" alt="Release">
  </a>
</p>

---

## چرا Qaren؟ <img src="../icons/favicon.png" width="24" height="24"> &nbsp; [<img src="../icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="../icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

ابزار استاندارد POSIX `diff` به مدت ۵۰ سال به ما خدمت کرده است، اما برای کد منبع طراحی شده بود، نه برای فایل‌های تنظیمات پیچیده و نسخه‌های پشتیبان سیستم حجیم امروزی که ترتیب در آن‌ها اهمیتی ندارد.

**Qaren (قارن)** یک ابزار چندمنظوره است که داده‌های شما را درک می‌کند.

- **تجزیه و تحلیل معنایی (Key-Value)**: ترتیب مهم نیست. قالب‌بندی مهم نیست. فقط داده‌ها مهم هستند.
- **امنیت Zero-Trust**: اطلاعات حساس مانند کلیدهای API، رمزهای عبور و رشته‌های اتصال به صورت خودکار مخفی می‌شوند (`***MASKED***`).
- **سرعت خیره‌کننده**: بهینه‌سازی شده با زبان Rust برای پردازش نسخه‌های پشتیبان سیستم چند گیگابایتی و بیش از ۱۰۰ هزار کلید، تا **۲۰۰ برابر سریع‌تر** از ابزارهای diff سنتی.
- **پشتیبانی از ANSI**: پاکسازی خودکار کدهای رنگ ترمینال از فایل‌های "آلوده" (مانند خروجی `pm2 env`) برای مقایسه‌ای دقیق.
- **وصله‌های هوشمند (Patching)**: ایجاد فایل‌های همگام‌سازی `.env` آماده برای محیط عملیاتی در چند ثانیه.

---

## <img src="../icons/icons8-doc-48.png" width="24" height="24"> مستندات
برای راهنماهای دقیق، مرجع API و تنظیمات پیشرفته، از سایت مستندات ما دیدن کنید:
> **[https://qaren.me/docs](https://qaren.me/docs)**

---

## <img src="../icons/icons8-feature-48.png" width="24" height="24"> ویژگی‌های کلیدی

### ۱. خروجی متنی بهبود یافته
Qaren مقایسه خط به خط بسیار واضح‌تری نسبت به POSIX diff ارائه می‌دهد که مخصوصاً برای تحلیل فایل‌های نسخه پشتیبان سیستم بهینه شده است.
```bash
$ qaren diff backup-old backup-new -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### ۲. حالت KV معنایی
درک فایل‌های `.env` ، `.yaml` و `.ini` بدون توجه به ترتیب کلیدها.
```bash
$ qaren kv prod.env staging.env
── Modified (1 keys) ──
  ~ PORT: 5000 → 4040
```

### ۳. کاهش نویز هوشمند
در حال مقایسه نسخه‌های پشتیبان JSON در حالت KV هستید؟ از `-D` برای مخفی کردن هشدارهای کلید تکراری و از `-P` برای بی‌صدا کردن هشدارهای دسترسی استفاده کنید. Qaren به طور خودکار تعداد هشدارها را به ۵ عدد برای هر فایل محدود می‌کند.

---

## <img src="../icons/icons8-installation-48.png" width="24" height="24"> نصب

```bash
# کلون کردن مخزن
git clone https://github.com/qaren-cli/qaren.git
cd qaren

# ساخت نسخه انتشار
cargo build --release

# فایل اجرایی در مسیر ./target/release/qaren در دسترس خواهد بود
```

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> نحوه استفاده و مثال‌ها

### مقایسه معنایی (KV)
```bash
# مقایسه پایه (تشخیص خودکار = یا :)
qaren kv file1.env file2.env

# مقایسه فرمت‌های مختلف (مثلاً .env در مقابل .yaml)
qaren kv file1.env file2.yaml --d2 ':'

# ایجاد فایل وصله (patch) برای کلیدهای مفقود
qaren kv prod.env local.env -g patch.env

# نادیده گرفتن کلیدها یا کلمات کلیدی خاص
qaren kv a.env b.env -x HOSTNAME --ignore-keyword AWS

# خروجی به فرمت JSON قابل خواندن توسط ماشین
qaren kv a.env b.env --output json
```

### مقایسه حرفی (Diff)
```bash
# فرمت unified diff (سازگار با POSIX)
qaren diff file1.txt file2.txt -u

# مقایسه بازگشتی دایرکتوری‌ها
qaren diff -r ./backup-old ./backup-new

# پاکسازی رنگ‌های ANSI قبل از مقایسه
qaren diff backup_polluted.txt backup_clean.txt -A

# نادیده گرفتن فواصل و خطوط خالی
qaren diff f1.txt f2.txt -w -B
```

---

## <img src="../icons/icons8-configuration-48.png" width="24" height="24"> تنظیمات

Qaren ترجیحات شما را به خاطر می‌سپارد.
```bash
# تغییر حالت مناسب برای خط لوله (خروجی همیشه 0)
qaren config exit toggle

# تغییر وضعیت نمایش رنگ در خروجی
qaren config color toggle

# مشاهده تنظیمات فعلی
qaren config show
```

---

## <img src="../icons/icons8-performance-48.png" width="24" height="24"> بنچمارک‌های عملکرد
| سناریو | برنده | اختلاف |
| :--- | :--- | :--- |
| **نسخه‌های پشتیبان حجیم (100MB)** | **Qaren** | **+200 برابر** |
| **دایرکتوری‌های بازگشتی** | **Qaren** | **3 برابر** |
| **تغییرات انبوه (۱ میلیون خط)** | **Qaren** | **+50 برابر** |

---

## <img src="../icons/icons8-contribution-64.png" width="24" height="24"> مشارکت و پشتیبانی

ما **پذیرای مشارکت‌های شما هستیم!** چه رفع باگ باشد، چه پارسر جدید یا بهبود عملکرد، درخواست‌های pull شما خوش‌آمد گفته می‌شود.

<img src="../icons/icons8-star-.gif" width="20" height="20"> **لطفاً اگر این ابزار برایتان مفید بود، به پروژه ستاره بدهید!**

- **وب‌سایت رسمی**: [https://qaren.me/](https://qaren.me/)
- **مستندات کامل**: [https://qaren.me/docs](https://qaren.me/docs)
- **گزارش باگ**: به [https://qaren.me/community](https://qaren.me/community) بروید و روی **"Open Issue"** کلیک کنید.

---

## <img src="../icons/icons8-licence-48.png" width="24" height="24"> مجوز
این پروژه تحت **مجوز MIT** منتشر شده است. برای جزئیات بیشتر فایل `LICENSE` را مطالعه کنید.

---

<p align="right">(قارن) — با افتخار برای مهندسان ساخته شده است</p>
