# MD Example

### Структура проекта

- `src/` - основная директория с контентом
  - `SUMMARY.md` - файл навигации/оглавления
  - Все остальные `.md` файлы - страницы документации

### Расположение файлов

- Все Markdown файлы должны находиться в директории `src/`. Рекомендуется ссылаться на md файлы из SUMMARY.md (см. пример в SUMMARY.md).
- Изображения и другие ресурсы рекомендуется хранить в релеватных папках `src/images`, `src/diagrams` и тд.

### Локальная разработка

1. Установите MDBook [инструкция](https://rust-lang.github.io/mdBook/guide/installation.html)
2. Запустите сервер разработки:

```bash
mdbook serve --open
```
3. Откройте http://localhost:3000 в браузере.

### Конфигурация репозитория 

1. Сделайте fork репозитория [mdb](https://github.com/Jho00/system-engineering-playbook)
2. Перейдите в настройки своего github репозитория. Выберите вкладку Actions -> General. Установите "Read and write permissions" в категории Workflow permissions