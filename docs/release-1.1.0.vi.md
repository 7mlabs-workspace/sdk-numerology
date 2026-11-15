# Numerology SDK 1.1.0

**Tiếng Việt** · [English](release-1.1.0.md)

[Trang SDK](../README.md) · [Công thức](features-and-formulas.vi.md) · [Quy tắc](rules.vi.md) · [Payload có lọc](payload-filtering.vi.md)

Ngày phát hành: 15/11/2026. Giấy phép MIT · Copyright (c) 2026 7mlabs.

## Thay đổi chính

1.1.0 bổ sung API xuất payload có lọc, cùng lõi Rust cho Node.js/TypeScript, Python, .NET, C và CLI. Đây là bản nâng cấp giữ tương thích. Sáu operation, công thức Pythagorean và chuẩn hóa tên Việt giữ nguyên.

## So sánh 1.0.0 và 1.1.0

| Nội dung | 1.0.0 | 1.1.0 |
| --- | --- | --- |
| `calculate` và các hàm đầy đủ | Schema 1.0/1.1, có dữ liệu nguồn | Giữ cấu trúc, phép tính và lỗi; engineVersion tăng lên 1.1.0 |
| API xuất có lọc | Chưa có | Thêm `calculateFiltered` và hàm tương đương; schema riêng `filtered-1.0` |
| Tổng gốc và bước rút gọn | Có trong payload đầy đủ | Giữ trong payload đầy đủ và các chỉ số được phép xuất có lọc |
| Master / số rút gọn | `reducedValue` qua extension hoặc operation names | Giữ cách cũ; mọi Number ở API có lọc luôn có reducedValue |
| Tên và chữ cái nguồn | Có ở các operation phù hợp | Giữ ở API đầy đủ; lược khỏi đầu ra có lọc |
| Thành phần ngày sinh, biểu đồ nguồn, năm lịch theo tuổi | Có ở các operation phù hợp | Giữ ở API đầy đủ; lược các nhóm này khỏi đầu ra có lọc |
| Trace Tư duy / Thái độ | Có khi chỉ số được tính | API đầy đủ giữ nguyên; API có lọc giữ kết quả nhưng lược root/steps và ghi lý do |
| ID tên/nhóm, cảnh báo | ID do caller cung cấp | API có lọc dùng nhãn cục bộ và cảnh báo có cấu trúc, không sao chép ID vào thông báo |
| Quy tắc trong payload | rulesVersion, options, metadata extension | API có lọc thêm calculationRules và effectiveRules theo từng Number |
| Công thức và chuẩn hóa tên Việt | Bộ quy tắc Pythagorean 7mlabs 1.0 | Không đổi |

## Nâng gói và chuyển API

Code đang gọi `calculate` không cần đổi cách gọi. Nâng phiên bản **không tự bật bộ lọc**. Ứng dụng cần hạn chế dữ liệu xuất phải chủ động gọi API mới và tránh ghi request/payload đầy đủ vào log.

```js
const { calculate, calculateFiltered } = require('@7mlabs/numerology');
const request = { operation: 'name', name: { given: 'Đặng' } };
const full = calculate(request);
const filtered = calculateFiltered(request);
// Cùng expression: root=17, steps=[17,8], value=8.
// Chỉ full có normalizedName; filtered dùng schema filtered-1.0.
```

| Binding | Hàm mới |
| --- | --- |
| Node.js | calculateFiltered / calculateFilteredJson |
| Python | calculate_filtered / calculate_filtered_json |
| .NET | CalculateFiltered / CalculateFilteredJson |
| Rust | calculate_filtered, calculate_names_filtered, calculate_filtered_json/bytes |
| C ABI v1 | numerology_calculate_filtered_json / numerology_calculate_filtered_json_len |
| CLI | --filtered |

## Tổng gốc, master và ngoại lệ

Ví dụ tổng 38: giữ master trả `root=38`, `steps=[38,11]`, `value=11`, `reducedValue=2`. Tắt master trả `steps=[38,11,2]`, `value=2`, `isMaster=false`. Bộ lọc không sửa phép tính hoặc tạo trace giả.

Tư duy và Thái độ có ngoại lệ ở đầu ra có lọc: tổng Tư duy trừ tổng tên có thể ra ngày sinh; tổng Thái độ trừ ngày sinh có thể ra tháng sinh. Hai chỉ số giữ kết quả nhưng bỏ root/steps, kèm `traceStatus=withheld` và `traceWithheldReason=birthDateInference`. Xem [chính sách từng trường](payload-filtering.vi.md).

## Công thức và quy tắc được giữ nguyên

Vẫn dùng `pythagorean-7mlabs-1.0` cùng các rulesVersion extension hiện có. Unicode NFD, bỏ dấu kết hợp, Đ/đ → D, chữ Y theo cấu hình và tên given/middle/family đều giữ cách xử lý cũ. Không tự chuyển sang công thức của nhà cung cấp khác hoặc tính các phím gõ Telex như chữ trong tên.

Xem [toàn bộ chức năng và công thức](features-and-formulas.vi.md), [quy tắc và ca biên](rules.vi.md), [nguồn kiểm chứng](formula-verification.vi.md).

## Nền tảng và kiểm chứng

Ba hệ gói: npm `@7mlabs/numerology`, PyPI `sevenmlabs-numerology`, NuGet `SevenMLabs.Numerology`. Native được kiểm chứng cho macOS ARM64 và Linux x64; Python có hai wheel. Các giới hạn nền tảng khác giữ nguyên.

Kiểm thử gồm hợp đồng cũ đóng băng, 417 yêu cầu đối chiếu trực tiếp với binary 1.0.0, 66 kiểm thử Rust với ma trận 2.700 cấu hình, schema có lọc, từng root/steps và consumer cài gói thật trên hai nền tảng. Bản kê phát hành ghi commit và SHA-256 của đúng các tệp đã qua CI. So với 1.0.0, phép so sánh tương thích chỉ cho phép engineVersion đổi.

Đầu ra có lọc giảm dữ liệu, **không phải dữ liệu ẩn danh**. Tổ hợp chỉ số vẫn có thể giúp nhận diện; trace chu kỳ vẫn có thể dựng lại ngày tham chiếu và được metadata ghi rõ.
