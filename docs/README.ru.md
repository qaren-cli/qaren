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
  <b>Следующее поколение инструментов сравнения конфигураций и системных бэкапов.</b><br>
  Создано для современной эпохи DevOps: семантично, безопасно и невероятно быстро.
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

## Почему Qaren? <img src="../icons/favicon.png" width="24" height="24"> &nbsp; [<img src="../icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="../icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

Стандартный POSIX `diff` служит нам уже 50 лет, но он был разработан для исходного кода, а не для сложных, не зависящих от порядка конфигурационных файлов и огромных системных бэкапов современности.

Qaren (с арабского **«Сравнивать»**) — это многопарадигмальный инструмент, который понимает ваши данные.

- **Семантический парсинг Key-Value**: Порядок не имеет значения. Форматирование не имеет значения. Важны только данные.
- **Безопасность Zero-Trust**: Секреты, такие как API-ключи, пароли и строки подключения, маскируются по умолчанию (`***MASKED***`).
- **Невероятная скорость**: Оптимизировано на Rust для обработки системных бэкапов гигабайтного масштаба и более 100 000 ключей до **200 раз быстрее**, чем традиционные методы diff.
- **Поддержка ANSI**: Автоматически очищает файлы от кодов цвета терминала (например, вывод `pm2 env`) для чистого сравнения.
- **Интеллектуальные патчи**: Создавайте готовые к продакшену патчи `.env` для синхронизации сред за считанные секунды.

---

## <img src="../icons/icons8-doc-48.png" width="24" height="24"> Документация
Подробные руководства, справочник API и расширенная конфигурация доступны в нашей документации:
> **[https://qaren.me/docs](https://qaren.me/docs)**

---

## <img src="../icons/icons8-feature-48.png" width="24" height="24"> Ключевые особенности

### 1. Семантический режим KV
Понимает файлы `.env`, `.yaml` и `.ini` независимо от порядка ключей.
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="Семантический режим KV">
</p>

### 2. Улучшенный вывод
Qaren предоставляет гораздо более четкие построчные различия, чем POSIX diff, специально оптимизированные для анализа файлов бэкапов.
```bash
$ qaren diff backup-old backup-new -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 3. Умное шумоподавление
Сравниваете бэкапы JSON в режиме KV? Qaren автоматически скрывает предупреждения о дубликатах ключей и правах доступа по умолчанию. Если вам нужна помощь в отладке, запустите `qaren config advisor toggle`, чтобы включить полезные оповещения.

---

## <img src="../icons/icons8-installation-48.png" width="24" height="24"> Установка

### Быстрая установка (автоматически)

| Платформа | Команда |
| :--- | :--- |
| **Linux / macOS** | `curl -sSfL https://qaren.me/install | sh` |
| **Windows** | `irm https://qaren.me/install.ps1 | iex` |
| **Homebrew** | `brew tap qaren-cli/qaren && brew install qaren` |

### Альтернативные методы
```bash
# Через Cargo
cargo install qaren
```

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> Использование и примеры

Режим `kv` в Qaren разработан для реальных задач DevOps. Все следующие примеры протестированы с использованием данных, представленных в этих двух файлах окружения:

<p align="center">
  <img src="../icons/dev_env.svg" width="45%" alt="Среда разработки">
  <img src="../icons/staging_env.svg" width="45%" alt="Тестовая среда">
</p>

### 1. Базовый семантический diff
Сравните два файла семантически, игнорируя порядок строк.
```bash
qaren kv -Q --d2 ":" dev.env staging.env
```
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="Базовый семантический diff">
</p>

### 2. Режим сводки
Получите высокоуровневый обзор различий без подробных изменений по строкам.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -s
```
<p align="center">
  <img src="../icons/Qd2s.gif" width="100%" alt="Режим сводки">
</p>

### 3. Экспорт в JSON
Экспортируйте результаты в машиночитаемом формате для автоматизации.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -o json
```
<p align="center">
  <img src="../icons/Qd2o.gif" width="100%" alt="Экспорт в JSON">
</p>

### 4. Показать секреты
Обойдите автоматическое маскирование, чтобы увидеть необработанные конфиденциальные значения.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -S
```
<p align="center">
  <img src="../icons/Qd2S.gif" width="100%" alt="Показать секреты">
</p>

### 5. Игнорировать ключи
Исключите известные динамические или нерелевантные ключи из сравнения.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -x API_KEY
```
<p align="center">
  <img src="../icons/Qd2x.gif" width="100%" alt="Игнорировать ключи">
</p>

### 6. Игнорировать по ключевому слову
Исключите все ключи, содержащие определенную подстроку.
```bash
qaren kv --ignore-keyword MAX ...
```
<p align="center">
  <img src="../icons/Qd2-ignore-keyword.gif" width="100%" alt="Игнорировать по ключевому слову">
</p>

### 7. Тихий режим
Проверяйте совместимость в скриптах только через коды выхода.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -q
```
<p align="center">
  <img src="../icons/Qd2q.gif" width="100%" alt="Тихий режим">
</p>

### 8. Генерация патча
Создайте файл патча для синхронизации отсутствующих ключей.
```bash
qaren kv ... -g missing.env
```
<p align="center">
  <img src="../icons/Qd2g.gif" width="100%" alt="Генерация патча">
</p>

### 9. Безопасные патчи
Генерируйте патчи с автоматически замаскированными конфиденциальными данными.
```bash
qaren kv ... -g missing.env --mask-patches
```
<p align="center">
  <img src="../icons/Qd2g-masked.gif" width="100%" alt="Безопасные патчи">
</p>

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> Литеральное сравнение (Diff)
```bash
# Формат unified diff (совместим с POSIX)
qaren diff file1.txt file2.txt -u

# Рекурсивное сравнение директорий
qaren diff -r ./backup-old ./backup-new

# Очистка ANSI-цветов перед сравнением
qaren diff backup_polluted.txt backup_clean.txt -A

# Игнорировать пробелы и пустые строки
qaren diff f1.txt f2.txt -w -B
```

---

## <img src="../icons/icons8-configuration-48.png" width="24" height="24"> Конфигурация

Qaren запоминает ваши предпочтения.
<p align="center">
  <img src="../icons/config-color.gif" width="100%" alt="Переключение цвета">
</p>

```bash
# Переключить режим для пайплайнов (всегда выход 0)
qaren config exit toggle

# Переключить цветной вывод
qaren config color toggle

# Переключить Advisor (предупреждения)
qaren config advisor toggle

# Переключить маскировку секретов
qaren config masking toggle

# Показать текущие настройки
qaren config show
```

---

## <img src="../icons/icons8-performance-48.png" width="24" height="24"> Бенчмарки производительности
| Сценарий | Победитель | Разрыв |
| :--- | :--- | :--- |
| **Большие бэкапы (100MB)** | **Qaren** | **200x+** |
| **Рекурсия (директории)** | **Qaren** | **3x** |
| **Массовые изменения (1М строк)** | **Qaren** | **50x+** |

---

## <img src="../icons/icons8-contribution-64.png" width="24" height="24"> Участие и поддержка

Мы **открыты для контрибьюта!** Пожалуйста, прочтите наше **[руководство по участию](CONTRIBUTING.md)** перед отправкой Pull Request.

- [ ] **Форкните** репозиторий.
- [ ] **Улучшите** или **добавьте** функции (избегайте удалений).
- [ ] Убедитесь в **отсутствии ворнингов** (`clippy` и `tests`).
- [ ] Обновите **документацию** и **--help** для новых флагов.

<img src="../icons/icons8-star-.gif" width="20" height="20"> **Пожалуйста, поставьте звезду проекту, если он вам полезен!**

- **Официальный сайт**: [https://qaren.me/](https://qaren.me/)
- **Полная документация**: [https://qaren.me/docs](https://qaren.me/docs)
- **Отчеты об ошибках**: Перейдите на [https://qaren.me/community](https://qaren.me/community) и нажмите **"Open Issue"**.

---

## <img src="../icons/icons8-licence-48.png" width="24" height="24"> Лицензия
Этот проект распространяется под **лицензией MIT**. Подробности в файле `LICENSE`.

---

<p align="right">(قارن) — Сделано с гордостью для инженеров</p>
