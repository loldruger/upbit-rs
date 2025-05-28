#!/bin/bash

# crates.io 배포 스크립트

echo "=== crates.io 배포 가이드 ==="
echo ""

echo "1. crates.io 계정 생성:"
echo "   https://crates.io 에서 GitHub 계정으로 로그인"
echo ""

echo "2. API 토큰 생성:"
echo "   https://crates.io/me → Account Settings → API Tokens → New Token"
echo ""

echo "3. API 토큰 설정:"
echo "   cargo login your_api_token_here"
echo ""

echo "4. 패키지 검증:"
echo "   cargo check"
echo "   cargo test --lib (선택사항)"
echo ""

echo "5. 패키지 생성:"
echo "   cargo package"
echo ""

echo "6. 배포:"
echo "   cargo publish"
echo ""

echo "=== 배포 전 체크리스트 ==="
echo "✅ Cargo.toml에 필수 정보 입력 완료"
echo "✅ README.md 작성 완료"
echo "✅ 라이센스 파일 확인"
echo "✅ .gitignore에 .env 파일 제외 설정"
echo "✅ 빌드 및 기본 테스트 통과"
echo ""

echo "주의사항:"
echo "- 한번 배포한 버전은 삭제할 수 없습니다"
echo "- 새 버전을 배포하려면 Cargo.toml의 version을 올려야 합니다"
echo "- API 키가 포함된 파일(.env)은 절대 배포되지 않도록 주의하세요"
