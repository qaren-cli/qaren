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
  <a href="README.ja.md">日本語</a> | 
  <a href="README.de.md">Deutsch</a> | 
  <a href="README.fr.md">Français</a>
</p>

<p align="center">
  <b>نسل جدید ابزارهای مقایسه تنظیمات و نسخه‌های پشتیبان سیستم.</b><br>
  طراحی شده برای عصر مدرن DevOps: معنایی (Semantic)، امن، و فوق‌العاده سریع.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-1.0.1-orange.svg" alt="Version">
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

### ۱. حالت KV معنایی
درک فایل‌های `.env` ، `.yaml` و `.ini` بدون توجه به ترتیب کلیدها.
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="Semantic KV Mode">
</p>

### ۲. خروجی متنی بهبود یافته
Qaren مقایسه خط به خط بسیار واضح‌تری نسبت به POSIX diff ارائه می‌دهد که مخصوصاً برای تحلیل فایل‌های نسخه پشتیبان سیستم بهینه شده است.
```bash
$ qaren diff backup-old backup-new -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### ۳. کاهش نویز هوشمند
در حال مقایسه نسخه‌های پشتیبان JSON در حالت KV هستید؟ Qaren به طور خودکار هشدارهای مربوط به کلیدهای تکراری و دسترسی‌ها را به صورت پیش‌فرض مخفی می‌کند تا خروجی ترمینال شما تمیز بماند. اگر برای عیب‌یابی به این هشدارها نیاز دارید، دستور `qaren config advisor toggle` را برای فعال‌سازی آن‌ها اجرا کنید.

---

## <img src="../icons/icons8-installation-48.png" width="24" height="24"> نصب

### نصب سریع (خودکار)

| پلتفرم | دستور |
| :--- | :--- |
| **Linux / macOS** | `curl -sSfL https://qaren.me/install | sh` |
| **Windows** | `irm https://qaren.me/install.ps1 | iex` |
| **Homebrew** | `brew tap qaren-cli/qaren && brew install qaren` |

### روش‌های جایگزین
```bash
# از طریق Cargo
cargo install qaren
```

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> نحوه استفاده و مثال‌ها

حالت `kv` در Qaren برای کارهای واقعی DevOps طراحی شده است. در اینجا الگوهای رایج برای مقایسه فایل‌های محیطی آورده شده است.

### ۱. مقایسه معنایی پایه
مقایسه معنایی دو فایل با نادیده گرفتن ترتیب خطوط.
```bash
qaren kv -Q --d2 ":" dev.env staging.env
```
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="Basic Semantic Diff">
</p>

### ۲. حالت خلاصه
دریافت یک نمای کلی از تفاوت‌ها بدون نمایش جزئیات تغییرات هر خط.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -s
```
<p align="center">
  <img src="../icons/Qd2s.gif" width="100%" alt="Summary Mode">
</p>

### ۳. خروجی JSON
استخراج نتایج در قالب قابل خواندن توسط ماشین برای خودکارسازی.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -o json
```
<p align="center">
  <img src="../icons/Qd2o.gif" width="100%" alt="JSON Export">
</p>

### ۴. نمایش اطلاعات حساس
نادیده گرفتن مخفی‌سازی خودکار برای مشاهده مقادیر حساس واقعی.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -S
```
<p align="center">
  <img src="../icons/Qd2S.gif" width="100%" alt="Show Secrets">
</p>

### ۵. نادیده گرفتن کلیدهای خاص
مستثنی کردن کلیدهای داینامیک یا غیرمرتبط از فرآیند مقایسه.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -x API_KEY
```
<p align="center">
  <img src="../icons/Qd2x.gif" width="100%" alt="Ignore Keys">
</p>

### ۶. نادیده گرفتن با استفاده از کلمه کلیدی
مستثنی کردن تمام کلیدهایی که شامل یک رشته خاص هستند.
```bash
qaren kv --ignore-keyword MAX ...
```
<p align="center">
  <img src="../icons/Qd2-ignore-keyword.gif" width="100%" alt="Ignore Keyword">
</p>

### ۷. حالت بیصدا
بررسی سازگاری در اسکریپت‌ها فقط از طریق کدهای خروجی (Exit Codes).
```bash
qaren kv -Q --d2 ":" dev.env staging.env -q
```
<p align="center">
  <img src="../icons/Qd2q.gif" width="100%" alt="Quiet Mode">
</p>

### ۸. تولید فایل وصله
ایجاد فایل وصله برای همگام‌سازی کلیدهای مفقود.
```bash
qaren kv ... -g missing.env
```
<p align="center">
  <img src="../icons/Qd2g.gif" width="100%" alt="Patch Generation">
</p>

### ۹. وصله‌های امن
تولید وصله‌ها در حالی که اطلاعات حساس به طور خودکار مخفی شده‌اند.
```bash
qaren kv ... -g missing.env --mask-patches
```
<p align="center">
  <img src="../icons/Qd2g-masked.gif" width="100%" alt="Secure Patches">
</p>

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> مقایسه حرفی (Diff)
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
<p align="center">
  <img src="../icons/config-color.gif" width="100%" alt="تغییر وضعیت رنگ">
</p>

```bash
# تغییر وضعیت به حالت مناسب برای خط لوله (همیشه خروجی 0)
qaren config exit toggle

# تغییر وضعیت نمایش رنگ
qaren config color toggle

# تغییر وضعیت مشاور (هشدارها)
qaren config advisor toggle

# تغییر وضعیت مخفی‌سازی اطلاعات حساس
qaren config masking toggle

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

ما **پذیرای مشارکت‌های شما هستیم!** لطفاً قبل از ارسال درخواست Pull، **[راهنمای مشارکت](CONTRIBUTING.md)** را مطالعه کنید.

- [ ] پروژه را **Fork** کنید.
- [ ] ویژگی‌ها را **بهبود** دهید یا ویژگی جدید **اضافه** کنید (از حذف کدها خودداری کنید).
- [ ] از **عدم وجود هشدار** اطمینان حاصل کنید (`clippy` و `tests`).
- [ ] **مستندات** و **--help** را برای فلگ‌های جدید بروزرسانی کنید.

<img src="../icons/icons8-star-.gif" width="20" height="20"> **لطفاً اگر این ابزار برایتان مفید بود، به پروژه ستاره بدهید!**

- **وب‌سایت رسمی**: [https://qaren.me/](https://qaren.me/)
- **مستندات کامل**: [https://qaren.me/docs](https://qaren.me/docs)
- **گزارش باگ**: به [https://qaren.me/community](https://qaren.me/community) بروید و روی **"Open Issue"** کلیک کنید.

---

## <img src="../icons/icons8-licence-48.png" width="24" height="24"> مجوز
این پروژه تحت **مجوز MIT** منتشر شده است. برای جزئیات بیشتر فایل `LICENSE` را مطالعه کنید.

---

<p align="right">(قارن) — با افتخار برای مهندسان ساخته شده است</p>
