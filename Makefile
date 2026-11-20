# Dùng cargo trong PATH, nếu không có thì lấy từ ~/.cargo/bin (rustup)
CARGO ?= $(shell command -v cargo 2>/dev/null || echo $(HOME)/.cargo/bin/cargo)
PORT  ?= 8080

.DEFAULT_GOAL := help
.PHONY: help run dev pdf build stop clean push

help: ## Liệt kê các lệnh
	@grep -E '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36mmake %-6s\033[0m %s\n", $$1, $$2}'

run: ## Chạy trang ở http://localhost:8080  (đổi cổng: make run PORT=3000)
	PORT=$(PORT) $(CARGO) run

dev: pdf run ## Tạo lại PDF rồi chạy trang (dùng sau khi sửa src/data.rs)

pdf: ## Tạo lại assets/Duong-Van-Hieu-CV.pdf từ trang (cần Chrome)
	$(CARGO) run -q -- pdf

build: ## Xuất trang tĩnh ra dist/ (giống bản deploy GitHub Pages)
	$(CARGO) run -q --release -- build

stop: ## Tắt server đang chạy (mặc định cổng 8080)
	@lsof -ti:$(PORT) | xargs kill 2>/dev/null && echo "Stopped :$(PORT)" || echo "Nothing running on :$(PORT)"

clean: ## Xoá target/ và dist/
	$(CARGO) clean
	rm -rf dist

push: ## Tạo lại PDF, commit và push (bắt buộc MSG="..."); GitHub Pages tự deploy
	@test -n "$(MSG)" || (echo 'Thiếu MSG, ví dụ: make push MSG="cv: update GHN"'; exit 1)
	$(MAKE) pdf
	git add -A
	git commit -m "$(MSG)"
	git push
