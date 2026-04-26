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
  <b>الجيل القادم من أدوات مقارنة الإعدادات (Configurations) والنسخ الاحتياطي للنظام (System Backups).</b><br>
  صُمم لعصر الـ DevOps الحديث: دلالي، آمن، وسريع للغاية.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-1.0.0-orange.svg" alt="Version">
  <img src="https://img.shields.io/badge/PRs-welcome-cyan.svg" alt="PRs Welcome">
  <a href="https://github.com/qaren-cli/qaren/actions/workflows/release.yml">
    <img src="https://github.com/qaren-cli/qaren/actions/workflows/release.yml/badge.svg" alt="Release">
  </a>
</p>

---

## لماذا قارن؟ <img src="../icons/favicon.png" width="24" height="24"> &nbsp; [<img src="../icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="../icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

أداة `diff` القياسية خدمت المهندسين لـ 50 عاماً، لكنها صُممت للشفرة المصدرية (Source Code)، وليس لملفات الإعدادات المعقدة والنسخ الاحتياطي الضخم للنظام الذي لا يعتمد على ترتيب الأسطر كما هو الحال اليوم.

**قارن (Qaren)** هي أداة متعددة الأنماط تفهم بياناتك.

- **تحليل دلالي (Key-Value)**: الترتيب لا يهم. التنسيق لا يهم. البيانات فقط هي ما يهم.
- **أمان مطلق (Zero-Trust)**: يتم إخفاء الأسرار مثل مفاتيح الـ API وكلمات المرور تلقائياً (`***MASKED***`).
- **سرعة فائقة**: مُحسَّن بلغة Rust للتعامل مع نسخ احتياطي بحجم جيجابايت وأكثر من 100 ألف مفتاح بسرعة تصل إلى **200 ضعف** أسرع من أدوات diff التقليدية.
- **دعم ANSI**: يقوم بتنظيف أكواد الألوان الخاصة بالطرفية تلقائياً من الملفات "الملوثة" (مثل مخرجات `pm2 env`) لمقارنة نظيفة.
- **رقع ذكية (Patching)**: إنشاء ملفات تزامن `.env` جاهزة للإنتاج لمطابقة البيئات في ثوانٍ.

---

## <img src="../icons/icons8-doc-48.png" width="24" height="24"> التوثيق
للاطلاع على الأدلة التفصيلية، ومرجع الـ API، والإعدادات المتقدمة، تفضل بزيارة موقع التوثيق الخاص بنا:
> **[https://qaren.me/docs](https://qaren.me/docs)**

---

## <img src="../icons/icons8-feature-48.png" width="24" height="24"> الميزات الرئيسية

### 1. مخرجات نصية محسنة
يوفر "قارن" مقارنة أسطر أوضح بكثير من POSIX diff، مصممة خصيصاً لتحليل ملفات النسخ الاحتياطي للنظام.
```bash
$ qaren diff backup-old backup-new -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 2. نمط KV الدلالي
يفهم ملفات `.env` و `.yaml` و `.ini` بغض النظر عن ترتيب المفاتيح.
```bash
$ qaren kv prod.env staging.env
── Modified (1 keys) ──
  ~ PORT: 5000 → 4040
```

### 3. تقليل الضجيج الذكي
هل تقارن نسخ احتياطي JSON في نمط KV؟ استخدم `-D` لإخفاء تحذيرات المفاتيح المتكررة و `-P` لإسكات تنبيهات الأذونات. يقوم "قارن" تلقائياً بتحديد عدد التحذيرات بـ 5 لكل ملف للحفاظ على نظافة الطرفية.

---

## <img src="../icons/icons8-installation-48.png" width="24" height="24"> التثبيت

```bash
# استنساخ المستودع
git clone https://github.com/qaren-cli/qaren.git
cd qaren

# بناء نسخة الإصدار
cargo build --release

# سيكون الملف التنفيذي متاحاً في ./target/release/qaren
```

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> الاستخدام وأمثلة

### المقارنة الدلالية (KV)
```bash
# مقارنة أساسية (يكتشف التلقائياً = أو :)
qaren kv file1.env file2.env

# مقارنة تنسيقات مختلفة (مثلاً .env ضد .yaml)
qaren kv file1.env file2.yaml --d2 ':'

# إنشاء ملف رقعة (patch) للمفاتيح المفقودة
qaren kv prod.env local.env -g patch.env

# تجاهل مفاتيح أو كلمات مفتاحية معينة
qaren kv a.env b.env -x HOSTNAME --ignore-keyword AWS

# مخرجات بتنسيق JSON قابل للقراءة آلياً
qaren kv a.env b.env --output json
```

### المقارنة الحرفية (Diff)
```bash
# تنسيق unified diff (متوافق مع POSIX)
qaren diff file1.txt file2.txt -u

# مقارنة المجلدات بشكل متكرر
qaren diff -r ./backup-old ./backup-new

# مسح ألوان ANSI من ملفات النسخ الاحتياطي قبل المقارنة
qaren diff backup_polluted.txt backup_clean.txt -A

# تجاهل المسافات والأسطر الفارغة
qaren diff f1.txt f2.txt -w -B
```

---

## <img src="../icons/icons8-configuration-48.png" width="24" height="24"> الإعدادات

يتذكر "قارن" تفضيلاتك.
```bash
# تبديل نمط خطوط الأنابيب (الخروج دائماً بـ 0)
qaren config exit toggle

# تبديل مخرجات الألوان
qaren config color toggle

# عرض الإعدادات الحالية
qaren config show
```

---

## <img src="../icons/icons8-performance-48.png" width="24" height="24"> اختبارات الأداء
| السيناريو | الفائز | الفرق |
| :--- | :--- | :--- |
| **نسخ احتياطي ضخم (100MB)** | **قارن (Qaren)** | **200x+** |
| **المجلدات المتكررة** | **قارن (Qaren)** | **3x** |
| **تغييرات هائلة (مليون سطر)** | **قارن (Qaren)** | **50x+** |

---

## <img src="../icons/icons8-contribution-64.png" width="24" height="24"> المساهمة والدعم

نحن **نرحب بالمساهمات!** سواء كان ذلك إصلاحاً لخطأ، أو محللاً جديداً، أو تحسيناً للأداء، فإن طلبات السحب (PRs) الخاصة بك مرحب بها.

<img src="../icons/icons8-star-.gif" width="20" height="20"> **يرجى إعطاء نجمة للمشروع إذا وجدته مفيداً!**

- **الموقع الرسمي**: [https://qaren.me/](https://qaren.me/)
- **التوثيق الكامل**: [https://qaren.me/docs](https://qaren.me/docs)
- **تقارير الأخطاء**: انتقل إلى [https://qaren.me/community](https://qaren.me/community) واضغط على **"Open Issue"**.

---

## <img src="../icons/icons8-licence-48.png" width="24" height="24"> الترخيص
هذا المشروع مرخص بموجب **رخصة MIT**. راجع ملف `LICENSE` لمزيد من التفاصيل.

---

<p align="right">(قارن) — صنع بكل فخر للمهندسين</p>
