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
  <b>الجيل القادم من أدوات مقارنة الإعدادات (Configurations) والنسخ الاحتياطي للنظام (System Backups).</b><br>
  صُمم لعصر الـ DevOps الحديث: دلالي، آمن، وسريع للغاية.
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

## لماذا قارن؟ [<img src="../icons/favicon.png" width="24" height="24">](https://qaren.me) &nbsp; [<img src="../icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="../icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

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

### 1. نمط KV الدلالي
يفهم ملفات `.env` و `.yaml` و `.ini` بغض النظر عن ترتيب المفاتيح.
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="Semantic KV Mode">
</p>

### 2. مخرجات نصية محسنة
يوفر "قارن" مقارنة أسطر أوضح بكثير من POSIX diff، مصممة خصيصاً لتحليل ملفات النسخ الاحتياطي للنظام.

<p align="center">
  <b>أداة Diff التقليدية (POSIX)</b><br>
  <img src="../icons/diff.gif" width="100%" alt="Traditional POSIX Diff">
</p>

<p align="center">
  <b>مقارنة قارن المحسنة (Qaren)</b><br>
  <img src="../icons/qaren-diff.gif" width="100%" alt="Qaren Enhanced Diff">
</p>

### 3. تقليل الضجيج الذكي
يقوم "قارن" تلقائياً بإخفاء تحذيرات المفاتيح المتكررة وتنبيهات الأذونات بشكل افتراضي للحفاظ على نظافة الطرفية. إذا كنت بحاجة للمساعدة في استكشاف الأخطاء وإصلاحها، قم بتشغيل `qaren config advisor toggle` لتمكين التنبيهات المفيدة.

---

## <img src="../icons/icons8-installation-48.png" width="24" height="24"> التثبيت

### التثبيت السريع (آلي)

| المنصة | الأمر |
| :--- | :--- |
| **Linux / macOS** | `curl -sSfL https://qaren.me/install | sh` |
| **Windows** | `irm https://qaren.me/install.ps1 | iex` |
| **Homebrew** | `brew tap qaren-cli/qaren && brew install qaren` |

### طرق بديلة
```bash
# عبر Cargo
cargo install qaren
```

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> الاستخدام وأمثلة

صُمم نمط `kv` في "قارن" لمهام DevOps الواقعية. تم اختبار جميع الأمثلة التالية باستخدام البيانات الموضحة في ملفي البيئة هذين:

<p align="center">
  <img src="../icons/dev_env.svg" width="45%" alt="بيئة التطوير">
  <img src="../icons/staging_env.svg" width="45%" alt="بيئة الاختبار">
</p>

### 1. مقارنة دلالية أساسية
مقارنة ملفين دلالياً مع تجاهل ترتيب الأسطر.
```bash
qaren kv -Q --d2 ":" dev.env staging.env
```
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="Basic Semantic Diff">
</p>

### 2. نمط الملخص
احصل على نظرة عامة عالية المستوى على الاختلافات دون تغييرات تفصيلية في الأسطر.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -s
```
<p align="center">
  <img src="../icons/Qd2s.gif" width="100%" alt="Summary Mode">
</p>

### 3. تصدير JSON
تصدير النتائج بتنسيق قابل للقراءة آلياً للأتمتة.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -o json
```
<p align="center">
  <img src="../icons/Qd2o.gif" width="100%" alt="JSON Export">
</p>

### 4. إظهار الأسرار
تجاوز الإخفاء التلقائي لرؤية القيم الحساسة الخام.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -S
```
<p align="center">
  <img src="../icons/Qd2S.gif" width="100%" alt="Show Secrets">
</p>

### 5. تجاهل مفاتيح معينة
استبعاد المفاتيح الديناميكية المعروفة أو غير ذات الصلة من المقارنة.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -x API_KEY
```
<p align="center">
  <img src="../icons/Qd2x.gif" width="100%" alt="Ignore Keys">
</p>

### 6. تجاهل بواسطة الكلمة المفتاحية
استبعاد جميع المفاتيح التي تحتوي على سلسلة فرعية محددة.
```bash
qaren kv --ignore-keyword MAX ...
```
<p align="center">
  <img src="../icons/Qd2-ignore-keyword.gif" width="100%" alt="Ignore Keyword">
</p>

### 7. النمط الهادئ
التحقق من التوافق في البرامج النصية عبر أكواد الخروج فقط.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -q
```
<p align="center">
  <img src="../icons/Qd2q.gif" width="100%" alt="Quiet Mode">
</p>

### 8. إنشاء رقعة (Patch)
إنشاء ملف رقعة لمزامنة المفاتيح المفقودة.
```bash
qaren kv ... -g missing.env
```
<p align="center">
  <img src="../icons/Qd2g.gif" width="100%" alt="Patch Generation">
</p>

### 9. رقع آمنة
إنشاء رقع مع إخفاء البيانات الحساسة تلقائياً.
```bash
qaren kv ... -g missing.env --mask-patches
```
<p align="center">
  <img src="../icons/Qd2g-masked.gif" width="100%" alt="Secure Patches">
</p>

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> المقارنة الحرفية (Diff)
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
<p align="center">
  <img src="../icons/config-color.gif" width="100%" alt="Config Color Toggle">
</p>

```bash
# تبديل نمط خطوط الأنابيب (الخروج دائماً بـ 0)
qaren config exit toggle

# تبديل مخرجات الألوان
qaren config color toggle

# تبديل المستشار (التحذيرات)
qaren config advisor toggle

# تبديل إخفاء الأسرار
qaren config masking toggle

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

نحن **نرحب بالمساهمات!** يرجى قراءة **[دليل المساهمة](CONTRIBUTING.md)** قبل تقديم طلب سحب (Pull Request).

- [ ] **Fork** للمستودع.
- [ ] **تحسين** أو **إضافة** ميزات (تجنب الحذف).
- [ ] التأكد من **عدم وجود تحذيرات** (`clippy` و `tests`).
- [ ] تحديث **التوثيق** و **--help** للأعلام (flags) الجديدة.

<img src="../icons/icons8-star-.gif" width="20" height="20"> **يرجى إعطاء نجمة للمشروع إذا وجدته مفيداً!**

- **الموقع الرسمي**: [https://qaren.me/](https://qaren.me/)
- **التوثيق الكامل**: [https://qaren.me/docs](https://qaren.me/docs)
- **تقارير الأخطاء**: انتقل إلى [https://qaren.me/community](https://qaren.me/community) واضغط على **"Open Issue"**.

---

## <img src="../icons/icons8-licence-48.png" width="24" height="24"> الترخيص
هذا المشروع مرخص بموجب **رخصة MIT**. راجع ملف `LICENSE` لمزيد من التفاصيل.

---

<p align="right">(قارن) — صنع بكل فخر للمهندسين</p>
