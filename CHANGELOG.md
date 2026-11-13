# Changelog

## 1.1.0 — Unreleased / Chưa phát hành

- Thêm đầu ra `filtered-1.0` dùng chung từ Rust qua Node.js/TypeScript, Python, .NET, C ABI v1 và CLI `--filtered`; các hàm đầy đủ cũ giữ nguyên.
- Giữ tổng gốc, từng bước rút gọn, master và số hiển thị rút gọn. Riêng Thái độ/Tư duy lược trace để chặn các đường suy ngược ngày sinh đã xác định; vẫn giữ kết quả.
- Lọc tên/chữ cái, nguồn ngày sinh, biểu đồ nguồn và năm lịch theo tuổi; ánh xạ ID tên/nhóm sang nhãn cục bộ, tái tạo cảnh báo không kèm ID.
- Khai báo quy tắc hiệu lực theo Number; schema JSON đóng và kiểu TypeScript riêng. Không thay công thức hoặc chuẩn hóa tên Việt, không thêm lựa chọn cách tính/SVG.
- Bổ sung kiểm thử tương thích, trace, lọc trường, tái nhận diện đã biết, nhóm ghép, ca biên và consumer các gói. Đầu ra có lọc giảm dữ liệu, không phải dữ liệu ẩn danh.

Additive filtered APIs and versioned output schema. Existing full-output payloads retain their shape and arithmetic; engineVersion follows 1.1.0. The filtered contract preserves eligible raw totals/reduction steps, explicitly withholds two date-revealing traces, and never claims anonymization.

## 1.0.0 — 2026-11-11

Bản SDK thần số học Pythagorean độc lập đầu tiên của 7mlabs, giấy phép MIT.

- Một lõi Rust ngoại tuyến dùng chung cho Node.js/TypeScript, Python, .NET, C ABI và JSON CLI.
- Sáu chức năng: profile, name, birth, cycles, essence và names; chỉ số tên/ngày sinh, Trưởng thành, Cầu nối, chu kỳ cá nhân/thế giới, bốn Đỉnh cao, bốn Thử thách ngày sinh, chuyển chữ và Essence.
- Phần mở rộng tùy chọn: số master và số rút gọn hiển thị, Tư duy, ba Giai đoạn, trục biểu đồ, tên thường gọi và biểu đồ kết hợp.
- Nhiều tên với ID rõ ràng, nhóm ghép theo phần được chọn, tổng chữ gốc trước rút gọn và nguồn tính từng kết quả. Không tự thay chỉ số tên khai sinh.
- Hai Thử thách theo tên đã có căn cứ: Linh hồn và Nhân cách, với thứ tự họ tên bắt buộc, chữ đầu/cuối và chính sách số 0 rõ ràng. Các phương pháp chưa xác minh không được thêm.
- Giữ hợp đồng schema 1.0 và các lời gọi Rust/TypeScript trước phần mở rộng; chỉ thêm payload schema 1.1 khi yêu cầu rõ.
- Tài liệu Việt/Anh: công thức, quy tắc, căn cứ, ví dụ từ engine, logo đầy đủ, ảnh sáng/tối, mục lục và điều hướng.
- Quy trình phát hành kiểm tra gói cài thật, cùng commit và cùng bộ ca đối chiếu trên macOS ARM64 và Linux x64. Bản kê phát hành ghi hash các gói đã kiểm chứng.

Initial standalone MIT SDK. Versioned arithmetic conventions, explicit reference dates and configuration, reduction traces, strict input validation and compatibility fixtures. Chaldean, interpretation prose, compatibility scores, personality/career percentages and the unresolved website-specific formulas are outside this release.

Native availability is limited to the targets recorded in the release manifest. This SDK has a new JSON contract and is not a drop-in replacement for the earlier HTTP service.
