<a href="../README.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — logo đầy đủ"></a>

# Payload có lọc — SDK 1.1.0

**Tiếng Việt** · [English](payload-filtering.md)

[Trang SDK](../README.md) · [Công thức](features-and-formulas.vi.md) · [Quy tắc](rules.vi.md) · [Schema](../schemas/filtered-result.schema.json)

## Chọn đầu ra phù hợp

SDK 1.1.0 đang được chuẩn bị, chưa phát hành. Không đổi công thức hoặc chuẩn hóa tên Việt của 1.0.0.

| API | Đầu ra | Tương thích |
| --- | --- | --- |
| `calculate` / `calculateJson` | Payload đầy đủ schema 1.0 hoặc 1.1, có thể chứa tên và thành phần ngày sinh | Giữ cấu trúc, kết quả và lỗi cũ; `engineVersion` đổi theo phiên bản gói |
| `calculateFiltered` / `calculateFilteredJson` | Schema riêng `filtered-1.0`, hạn chế dữ liệu nguồn, giữ trace được phép xuất | Bổ sung riêng; không thay đổi hàm cũ |

Cùng sáu operation và cùng đầu vào. `extensions` tiếp tục quyết định chỉ số nào được tính; hàm có lọc không tự bật chỉ số mở rộng. Mọi Number xuất qua hàm mới luôn có `reducedValue`, kể cả khi đầu vào đặt `extensions.reducedValues=false`. Cờ đó chỉ điều khiển payload đầy đủ.

Đầu ra có lọc **không phải dữ liệu ẩn danh**, không phải cam kết đủ điều kiện công khai hoặc tuân thủ pháp luật. Tổng tên, tần suất, tuổi và tổ hợp chỉ số vẫn có thể giúp đối chiếu một người. `outputPolicy.anonymous` luôn là `false`.

## Tổng gốc, bước rút gọn và master

Một tên giả lập có tổng 38 và giữ master trả:

```json
{
  "root": 38,
  "steps": [38, 11],
  "value": 11,
  "reducedValue": 2,
  "isMaster": true,
  "karmicDebt": null,
  "traceStatus": "included",
  "effectiveRules": {
    "method": "wholeNameLetterSum",
    "reduction": "decimalDigitSum",
    "preserveMasterNumbers": [11, 22, 33]
  }
}
```

Tắt giữ master thì `steps=[38,11,2]`, `value=2`, `isMaster=false`. `reducedValue` phục vụ hiển thị 11/2, 22/4, 33/6; không phải tổng gốc. Số 0 vẫn là 0. `root`/`steps` được giữ phải trùng từng giá trị với phản hồi đầy đủ.

Ngoại lệ: Thái độ và mọi Tư duy (hồ sơ, từng tên, nhóm ghép) giữ `value`, `reducedValue`, `isMaster`, `karmicDebt` nhưng bỏ cả `root` và `steps`. Chúng có `traceStatus="withheld"` và `traceWithheldReason="birthDateInference"`. Không thay trace bằng 0, null hoặc các bước giả. Lý do: tổng Tư duy trừ tổng tên riêng/nhóm ra ngày sinh; tổng Thái độ trừ ngày đó ra tháng sinh. Tổng Thái độ 2 còn xác định ngày/tháng 1/1.

## Quy tắc lọc từng nhóm

| Nhóm trong payload đầy đủ | Schema có lọc |
| --- | --- |
| Các chỉ số tên, tổng gốc, bước rút gọn, tần suất, số thiếu, Đam mê tiềm ẩn, Tiềm thức | Giữ số liệu; bỏ `normalizedName`, `cornerstone`, `capstone` |
| Đường đời | Giữ Number đầy đủ và khai báo phương pháp |
| Thái độ | Giữ kết quả, bỏ trace như trên |
| `birth.components`, `birth.birthDay`, `birth.birthGrid` | Bỏ cả nhóm; ngay cả `birthDay.value=1` cũng có thể lộ ngày nguyên |
| Chu kỳ cá nhân/thế giới, Đỉnh cao và Thử thách ngày sinh | Giữ Number, trace, phase và dải tuổi |
| `extended.periodCycles`, `birthGridAxes`, `combinedGrid` | Bỏ cả nhóm; Period Cycles lặp nguồn ngày sinh, biểu đồ ghép trừ biểu đồ tên tái tạo biểu đồ ngày sinh |
| `extended.birthNameGridAxes` | Giữ trục của biểu đồ tên |
| Essence | Giữ `age`, Number Essence và trace; bỏ `birthYearPlusAge` cùng các transit `given/middle/family` chứa chữ và giá trị mã hóa tên |
| Thử thách theo tên | Giữ trạng thái, lý do thiếu dữ liệu, difference/value và phương pháp; bỏ `orderedText`, cả `first`/`last` (giá trị nguyên âm cũng có thể mã hóa duy nhất chữ cái) |
| Nhiều tên, tên thường gọi và nhóm ghép | Áp dụng cùng cách lọc; giữ phạm vi `part`, phương pháp ghép và liên kết bằng nhãn mới; bỏ `sources[].text` |
| ID tên/nhóm | Đổi theo thứ tự trong mỗi name set thành `n1`, `n2`, `c1`…; không trả ID cũ, bảng ánh xạ hoặc hash của tên/ngày sinh |
| Cảnh báo | Đối tượng `code/field/message` cố định, đường dẫn schema/chỉ số mảng; không chuyển tiếp chuỗi cảnh báo có ID từ payload đầy đủ |
| Lỗi | Cùng kiểm tra đầu vào, code/field và thông báo có kiểm soát; không đính kèm nội dung request hoặc lỗi parser thô |

Nhãn là cục bộ trong mỗi name set của một phản hồi; không phải định danh toàn cục hoặc token bảo mật. Ứng dụng tự gắn nhãn hiển thị trong phạm vi riêng khi cần. Không ghi nguyên request, payload đầy đủ hoặc bảng ánh xạ vào log rồi coi đầu ra đã lọc là đủ bảo vệ dữ liệu.

Trace chu kỳ vẫn có thể tái tạo **ngày tham chiếu**: `universalYear.root` là năm tham chiếu; các tổng tháng/ngày cho phép lấy lại tháng/ngày tham chiếu. `outputPolicy.referenceDateMayBeRecoverable=true` ghi rõ điều này khi có `data.cycles`. Đây là ngày tham chiếu, không phải năm sinh.

## Cách gọi

```js
const { calculateFiltered, calculate } = require('@7mlabs/numerology');
const request = { operation: 'name', name: { given: 'Đặng' } };
const result = calculateFiltered(request);
console.log(result.data.name.expression); // root=17, steps=[17,8], value=8
// Chỉ lấy bản đầy đủ trong luồng cần dữ liệu nguồn:
const full = calculate(request); // vẫn có normalizedName như 1.0.0
```

```python
from sevenmlabs_numerology import calculate_filtered
result = calculate_filtered({"operation": "name", "name": {"given": "Đặng"}})
```

```csharp
using SevenMLabs.Numerology;
var result = Numerology.CalculateFiltered("""{"operation":"name","name":{"given":"Đặng"}}""");
```

| Bề mặt | Hàm mới |
| --- | --- |
| Rust | `calculate_filtered(&Request)`, `calculate_names_filtered(&NamesRequest)`, `calculate_filtered_json`, `calculate_filtered_bytes`; JSON hỗ trợ đủ extension |
| Node.js/TypeScript | `calculateFiltered`, `calculateFilteredJson` |
| Python | `calculate_filtered`, `calculate_filtered_json` |
| .NET | `CalculateFiltered`, `CalculateFilteredJson` |
| C ABI v1 | `numerology_calculate_filtered_json_len`, `numerology_calculate_filtered_json`; giải phóng bằng `numerology_free` |
| CLI | `sevenmlabs-numerology-cli --filtered`, đọc JSON từ stdin |

Hàm JSON vẫn trả envelope `ok/result` hoặc `ok/error`. TypeScript dùng kiểu `FilteredResult` riêng. Không gán kết quả này vào kiểu payload đầy đủ hoặc giả định trường đã lược vẫn tồn tại.

## Quy tắc và kiểm chứng

`rulesVersion` và các mã extension giữ nguyên vì không đổi thuật toán. `calculationRules` nêu chuẩn hóa Unicode/Đ, phạm vi tên, chữ Y và cách tính Cầu nối. `effectiveRules` trên từng Number ghi phương pháp và chính sách giữ master thực tế, gồm các ngoại lệ luôn rút đơn.

Engine dựng đối tượng đầu ra từ danh sách trường cho phép. Trường mới ở payload đầy đủ không tự đi vào schema có lọc. Không sửa công thức, thêm trường phái, lựa chọn phép tính khác hoặc SVG trong bản này.

[Ví dụ đầu vào](examples/profile-request.json) · [Phản hồi đầy đủ](examples/profile-response.json) · [Phản hồi có lọc](examples/profile-filtered-response.json)

Kiểm thử gồm hợp đồng 1.0.0 đóng băng (chỉ cho phép đổi engineVersion), ma trận tên Việt/Unicode/ngày biên/chữ Y/master, nhóm ghép, giới hạn đầu vào, đường suy ngược đã biết, trường tương lai, schema đóng và các consumer native/cài gói thật. Mức kiểm chứng từng hệ điều hành phụ thuộc môi trường thực sự đã chạy, không suy từ một máy.
