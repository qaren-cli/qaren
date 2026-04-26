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
  <b>Следующее поколение инструментов сравнения конфигураций и логов.</b><br>
  Создано для современной эпохи DevOps: семантично, безопасно и невероятно быстро.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-0.3.6-orange.svg" alt="Version">
  <img src="https://img.shields.io/badge/PRs-welcome-cyan.svg" alt="PRs Welcome">
  <a href="https://github.com/qaren-cli/qaren/actions/workflows/release.yml">
    <img src="https://github.com/qaren-cli/qaren/actions/workflows/release.yml/badge.svg" alt="Release">
  </a>
</p>

---

## 🚀 Почему Qaren?

Стандартный POSIX `diff` служит нам уже 50 лет, но он был разработан для исходного кода, а не для сложных, не зависящих от порядка конфигурационных файлов и огромных логов современности.

Qaren (с арабского **«Сравнивать»**) — это многопарадигмальный инструмент, который понимает ваши данные.

- **Семантический парсинг Key-Value**: Порядок не имеет значения. Форматирование не имеет значения. Важны только данные.
- **Безопасность Zero-Trust**: Секреты, такие как API-ключи, пароли и строки подключения, маскируются по умолчанию (`***MASKED***`).
- **Невероятная скорость**: Оптимизировано на Rust для обработки логов гигабайтного масштаба и более 100 000 ключей до **200 раз быстрее**, чем традиционные методы diff.
- **Поддержка ANSI**: Автоматически очищает файлы от кодов цвета терминала (например, вывод `pm2 env`) для чистого сравнения.
- **Интеллектуальные патчи**: Создавайте готовые к продакшену патчи `.env` для синхронизации сред за считанные секунды.

---

## 📚 Документация
Подробные руководства, справочник API и расширенная конфигурация доступны в нашей документации:
👉 **[https://qaren.me/docs](https://qaren.me/docs)**

---

## 🛠️ Ключевые особенности

### 1. Улучшенный вывод
Qaren предоставляет гораздо более четкие построчные различия, чем POSIX diff, специально оптимизированные для анализа лог-файлов.
```bash
$ qaren diff old.log new.log -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 2. Семантический режим KV
Понимает файлы `.env`, `.yaml` и `.ini` независимо от порядка ключей.
```bash
$ qaren kv prod.env staging.env
── Modified (1 keys) ──
  ~ PORT: 5000 → 4040
```

### 3. Умное шумоподавление
Сравниваете логи JSON в режиме KV? Используйте `-D`, чтобы скрыть предупреждения о дубликатах ключей, и `-P`, чтобы отключить уведомления о правах доступа. Qaren автоматически ограничивает количество предупреждений до 5 на файл.

---

## 📥 Установка

```bash
# Клонировать репозиторий
git clone https://github.com/yourusername/qaren.git
cd qaren

# Собрать релизную версию
cargo build --release

# Бинарный файл будет доступен в ./target/release/qaren
```

---

## 📖 Использование и примеры

### Семантическое сравнение (KV)
```bash
# Базовое сравнение (автоопределение = или :)
qaren kv file1.env file2.env

# Сравнение разных форматов (например, .env и .yaml)
qaren kv file1.env file2.yaml --d2 ':'

# Создать файл патча для отсутствующих ключей
qaren kv prod.env local.env -g patch.env

# Игнорировать определенные ключи или ключевые слова
qaren kv a.env b.env -x HOSTNAME --ignore-keyword AWS

# Вывод в формате JSON
qaren kv a.env b.env --output json
```

### Литеральное сравнение (Diff)
```bash
# Формат unified diff (совместим с POSIX)
qaren diff file1.txt file2.txt -u

# Рекурсивное сравнение директорий
qaren diff -r ./logs-old ./logs-new

# Очистка ANSI-цветов перед сравнением
qaren diff logs_polluted.txt logs_clean.txt -A

# Игнорировать пробелы и пустые строки
qaren diff f1.txt f2.txt -w -B
```

---

## ⚙️ Конфигурация

Qaren запоминает ваши предпочтения.
```bash
# Переключить режим для пайплайнов (всегда выход 0)
qaren config exit toggle

# Переключить цветной вывод
qaren config color toggle

# Показать текущие настройки
qaren config show
```

---

## 📊 Бенчмарки производительности
| Сценарий | Победитель | Разрыв |
| :--- | :--- | :--- |
| **Большие логи (100MB)** | **Qaren** | **200x+** |
| **Рекурсия (директории)** | **Qaren** | **3x** |
| **Массовые изменения (1М строк)** | **Qaren** | **50x+** |

---

## 🤝 Участие и поддержка

Мы **открыты для контрибьюта!** Будь то исправление багов, новые парсеры или оптимизация производительности — ваши PR приветствуются.

⭐ **Пожалуйста, поставьте звезду проекту, если он вам полезен!**

- **Официальный сайт**: [https://qaren.me/](https://qaren.me/)
- **Полная документация**: [https://qaren.me/docs](https://qaren.me/docs)
- **Отчеты об ошибках**: Перейдите на [https://qaren.me/community](https://qaren.me/community) и нажмите **"Open Issue"**.

---

## 📜 Лицензия
Этот проект распространяется под **лицензией MIT**. Подробности в файле `LICENSE`.

---

<p align="right">(قارن) — Сделано с гордостью для инженеров</p>
