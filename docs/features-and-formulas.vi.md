<a href="../README.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — logo đầy đủ"></a>

# Chức năng và công thức tính toán

**Tiếng Việt** · [English](features-and-formulas.md)

[Trang SDK](../README.md) · **Chức năng & công thức** · [Quy tắc tính](rules.vi.md) · [Nguồn kiểm chứng](formula-verification.vi.md)

> SDK 1.1.0 bổ sung [đầu ra có lọc](payload-filtering.vi.md) với schema riêng; các bảng đường dẫn bên dưới mô tả payload đầy đủ. Công thức và quy tắc tính không đổi.

---

<details open>
<summary><strong>Trên trang này</strong></summary>

- [Trường phái và ký hiệu](#doc-section-1)
- [Chọn phạm vi tính](#doc-section-2)
- [Các chỉ số theo tên](#doc-section-3)
- [Ngày sinh và các chỉ số kết hợp](#doc-section-4)
- [Chu kỳ, Đỉnh cao và Thử thách ngày sinh](#doc-section-5)
- [Chuyển chữ theo tên và Essence](#doc-section-6)
- [Phần mở rộng tùy chọn](#doc-section-7)
- [Đọc đối tượng số và tái lập payload](#doc-section-8)
- [Phạm vi chưa hỗ trợ](#doc-section-9)
- [Mã nguồn và căn cứ](#doc-section-10)

</details>

Tài liệu mô tả **SDK 1.1.0 (candidate)**, gồm chức năng nhiều tên và phần mở rộng schema 1.1. Đối chiếu tệp gói với mã commit và mã SHA-256 trong bản kê phát hành. Xem [quy tắc chi tiết](rules.vi.md) để biết giới hạn đầu vào và trường hợp biên.

<a id="doc-section-1"></a>

## Trường phái và ký hiệu

Hiện chỉ triển khai **Pythagorean**. Không truyền `system` thì mặc định là `pythagorean`; truyền hệ khác sẽ nhận lỗi `UNSUPPORTED_SYSTEM`. Kết quả ghi trường phái, phiên bản engine, schema, bộ quy tắc và các tùy chọn thực tế. Mỗi phần mở rộng có mã quy tắc riêng. Đây là quy ước tính của 7mlabs, không khẳng định mọi tác giả hay website đều tính giống nhau.

- `R(n)`: cộng các chữ số thập phân lặp lại; dừng ở một chữ số hoặc ở 11, 22, 33 khi `keepMasterNumbers=true`. Khi tùy chọn này là false, `R` giống `D`.
- `D(n)`: cộng chữ số đến một chữ số, giữ 0 và không dừng ở số master.
- `V(text)`: tổng giá trị Pythagorean của các chữ đã chuẩn hóa; khoảng trắng chỉ ngăn từ, không có giá trị số.
- `LP`, `EX`, `SU`, `PE`: giá trị Đường đời, Sứ mệnh, Linh hồn, Nhân cách được trả về, bao gồm số master được giữ lại.
- `M`, `B`, `Y`: tháng, ngày, năm sinh đã rút về một chữ số: `D(tháng sinh)`, `D(ngày sinh)`, `D(năm sinh)`.
- `E = 36 - D(LP)`: tuổi cuối, tính cả tuổi đó, của Đỉnh cao và Giai đoạn đầu tiên.

| Giá trị | Chữ cái |
| --- | --- |
| 1 | A J S |
| 2 | B K T |
| 3 | C L U |
| 4 | D M V |
| 5 | E N W |
| 6 | F O X |
| 7 | G P Y |
| 8 | H Q Z |
| 9 | I R |

Tên được chuẩn hóa thành chữ Latin ASCII viết hoa, bỏ dấu có thể phân tách và đổi Đ thành D. Khoảng trắng, gạch nối, dấu nháy đơn là dấu ngăn từ. Các hệ chữ khác cần được bên gọi chuyển tự rõ ràng. AEIOU là nguyên âm; W là phụ âm.

Với Y, bên gọi chọn `vowel` hoặc `consonant`; mặc định `contextual` xem Y là nguyên âm khi không có AEIOU đứng liền trước hoặc sau trong cùng từ. Đây là quy tắc theo cách viết, không phải nhận diện phát âm.

<a id="doc-section-2"></a>

## Chọn phạm vi tính

Các giá trị `operation` là chức năng trong cùng một API JSON. Node.js/TypeScript, Python, .NET và CLI dùng chung lõi Rust. Các đường dẫn dưới đây tính từ đối tượng `result` đã mở; hàm JSON thô bọc bên ngoài bằng `{ok, result}` hoặc `{ok, error}`.

| Chức năng | Đầu vào bắt buộc | Kết quả |
| --- | --- | --- |
| `name` | `name` có các thành phần rõ ràng | `data.name`: một tên được cung cấp; có thể truyền tên thường gọi riêng. |
| `birth` | `birthDate` | `data.birth`: chỉ số từ ngày sinh và biểu đồ ngày sinh. |
| `cycles` | `birthDate`, `referenceDate` | `data.cycles`: sáu chu kỳ năm/tháng/ngày, bốn Đỉnh cao và bốn Thử thách từ ngày sinh. |
| `essence` | `name`, `birthDate` | `data.essence`: ba dòng chuyển chữ theo tên và Essence trong khoảng tuổi yêu cầu. |
| `profile` | `name`, `birthDate`, `referenceDate` | Toàn bộ bốn phạm vi cơ bản, `data.derived` và các phần mở rộng được bật. |
| `names` | Danh sách `names` không rỗng | `data.names`: từng tên riêng và các nhóm ghép do bên gọi chọn; có `birthDate` thì thêm Tư duy theo phạm vi. |

Trong tên, `given` bắt buộc; `middle` và `family` có thể bỏ trống và có thể chứa nhiều từ. Ngày theo lịch Gregory, dạng `YYYY-MM-DD`. Engine không lấy ngày từ đồng hồ máy. Với Essence/profile, `startAge` mặc định 0, `years` mặc định 100; các dòng phải nằm trong tuổi 0–120. Xem [giới hạn đầu vào](rules.vi.md#dau-vao).

<a id="doc-section-3"></a>

## Các chỉ số theo tên

Tên đầy đủ gồm mọi thành phần given/middle/family đã cung cấp. Tên thường gọi và mỗi tên trong danh sách dùng cùng công thức trên phạm vi riêng. Nhóm ghép dùng chuỗi đã chọn làm một thành phần given tổng hợp; không thay thế tên khai sinh trong hồ sơ.

| Chỉ số / dữ liệu | Công thức và phạm vi | Trường trong `data.name` |
| --- | --- | --- |
| Sứ mệnh — Expression | `R(V(tên đầy đủ))` | `expression` |
| Linh hồn — Soul Urge | `R(tổng giá trị nguyên âm của tên đầy đủ)`, theo tùy chọn Y | `soulUrge` |
| Nhân cách — Personality | `R(tổng giá trị phụ âm của tên đầy đủ)` | `personality` |
| Cân bằng — Balance | `R(tổng giá trị chữ đầu của từng từ trong mọi thành phần tên)` | `balance` |
| Chỉ số tên | `R(V(given))`, tính mọi từ trong thành phần given | `givenName` |
| Chỉ số họ | `R(V(family))`; không có họ thì bằng 0 | `familyName` |
| Biểu đồ tên | Đếm số lần xuất hiện của giá trị 1–9 trong tên đầy đủ | `frequency` |
| Số thiếu / bài học theo số thiếu | Các giá trị 1–9 có tần suất bằng 0 | `missingNumbers` |
| Đam mê tiềm ẩn — Hidden Passion | Mọi giá trị cùng có tần suất cao nhất, giữ cả trường hợp đồng hạng | `hiddenPassion` |
| Tiềm thức — Subconscious Self | `9 - số lượng giá trị thiếu` | `subconsciousSelf` |
| Chữ đầu — Cornerstone | Chữ đầu của thành phần given đã chuẩn hóa | `cornerstone` |
| Chữ cuối — Capstone | Chữ cuối của thành phần given đã chuẩn hóa | `capstone` |

Với **chỉ số tính tổng** Linh hồn/Nhân cách, không có nguyên âm/phụ âm thì tổng bằng 0. Quy ước này khác trạng thái không tính được của Thử thách theo chữ đầu–cuối ở phần sau.

<a id="doc-section-4"></a>

## Ngày sinh và các chỉ số kết hợp

| Chỉ số / dữ liệu | Công thức và phạm vi | Đường dẫn |
| --- | --- | --- |
| Đường đời — Life Path | `R(R(tháng sinh) + R(ngày sinh) + R(năm sinh))`; chính sách master áp dụng cho từng thành phần và tổng cuối | `data.birth.lifePath` |
| Ngày sinh — Birthday | `R(ngày sinh)` | `data.birth.birthDay` |
| Thái độ — Attitude | `D(tháng sinh + ngày sinh)` | `data.birth.attitude` |
| Thành phần ngày sinh | `R(tháng sinh)`, `R(ngày sinh)`, `R(năm sinh)` | `data.birth.components` |
| Biểu đồ ngày sinh | Đếm chữ số khác 0 của chuỗi Gregory `YYYYMMDD`, có đệm số 0; bỏ qua các số 0 | `data.birth.birthGrid` |
| Trưởng thành — Maturity | `R(LP + EX)` từ ngày sinh và tên khai sinh của hồ sơ | `data.derived.maturity` |
| Cầu nối Đường đời–Sứ mệnh | `abs(LP - EX)`, dùng giá trị đã trả về | `data.derived.lifePathExpressionBridge` |
| Cầu nối Linh hồn–Nhân cách | `abs(SU - PE)`, dùng giá trị đã trả về | `data.derived.soulPersonalityBridge` |

`abs` là trị tuyệt đối. Hai Cầu nối là hiệu trực tiếp, không rút lại bằng `R` hay `D`. Ví dụ `LP=9`, `EX=11` cho Cầu nối 2; không tự đổi 11 thành 2 rồi trả 7. Trưởng thành là phép tính riêng. Chưa xác minh được Cầu nối tương đương nhãn “Năng lực trưởng thành” của website đối chiếu, nên không gán nhãn đó.

<a id="doc-section-5"></a>

## Chu kỳ, Đỉnh cao và Thử thách ngày sinh

Gọi `ry`, `rm`, `rd` là năm, tháng, ngày nguyên gốc của ngày tham chiếu do bên gọi cung cấp.

| Chu kỳ | Công thức | Trường trong `data.cycles` |
| --- | --- | --- |
| Năm cá nhân | `D(M + B + D(ry))` | `personalYear` |
| Tháng cá nhân | `D(Năm cá nhân + rm)` | `personalMonth` |
| Ngày cá nhân | `D(Tháng cá nhân + rd)` | `personalDay` |
| Năm thế giới | `D(ry)` | `universalYear` |
| Tháng thế giới | `D(Năm thế giới + rm)` | `universalMonth` |
| Ngày thế giới | `D(Tháng thế giới + rd)` | `universalDay` |

Cả sáu đều không giữ master. Năm cá nhân dùng năm dương lịch tháng 1–12, không đổi chu kỳ vào sinh nhật. Tổng gốc và các bước rút gọn dùng đúng tháng/ngày tham chiếu nguyên gốc như công thức trên.

| Đỉnh cao | Công thức | Khoảng tuổi nguyên, tính cả hai đầu |
| --- | --- | --- |
| 1 | `P1 = R(M + B)` | `0..E` |
| 2 | `P2 = R(B + Y)` | `E+1..E+9` |
| 3 | `P3 = R(P1.value + P2.value)` | `E+10..E+18` |
| 4 | `P4 = R(M + Y)` | Từ `E+19`, `endAge=null` |

Kết quả ở `data.cycles.pinnacles`; `phase` đánh số từ 1. Các thành phần M/B/Y đầu vào đều là một chữ số, dù phần `data.birth.components` có giữ master. Đỉnh cao thứ ba cộng **giá trị đã trả về** của hai Đỉnh cao trước, bao gồm master được giữ lại.

Bốn Thử thách ngày sinh tại `data.cycles.challenges`, theo thứ tự: `C1=abs(M-B)`, `C2=abs(B-Y)`, `C3=abs(C1-C2)`, `C4=abs(M-Y)`. Chấp nhận 0, không giữ master. Đây là bốn phép tính khác với hai Thử thách theo tên. Khoảng tuổi nguyên không phải ngày chuyển giai đoạn chính xác hoặc kết luận về giai đoạn hiện tại.

<a id="doc-section-6"></a>

## Chuyển chữ theo tên và Essence

Với mỗi thành phần given/middle/family riêng: bỏ dấu ngăn từ, cho mỗi chữ chiếm số vị trí tuổi nguyên bằng giá trị 1–9 của chữ, chuyển sang chữ tiếp theo rồi lặp lại cả chuỗi. Ở tuổi `a`, vị trí là `a mod V(thành phần)`. Tuổi 0 bắt đầu bằng chữ đầu. Thành phần tùy chọn rỗng trả `{letter:null, value:0}`.

Mỗi dòng có `Essence = R(giá trị chuyển chữ given + middle + family)`. Dữ liệu ở `data.essence.years`, kèm chữ và giá trị đang hoạt động của từng thành phần. `birthYearPlusAge = năm sinh + tuổi` chỉ là nhãn năm bằng phép cộng, không phải khoảng ngày hoặc chuyển đổi múi giờ.

<a id="doc-section-7"></a>

## Phần mở rộng tùy chọn

Bỏ `extensions` hoặc truyền null giữ schema 1.0. Truyền đối tượng sẽ chọn schema 1.1. `reducedValues` và `indicators` mặc định true, có thể tắt độc lập. Các giá trị và trường cũ giữ nguyên ý nghĩa.

| Phần mở rộng | Công thức / cách hoạt động | Vị trí và điều kiện |
| --- | --- | --- |
| Số rút gọn để hiển thị | `reducedValue = D(value)`; không sửa `value`, `steps`, `isMaster` | Thêm vào đối tượng số khi `reducedValues=true`; `names` độc lập luôn có trường này |
| Tư duy — Rational Thought | `D(V(birthName.given) + ngày sinh nguyên gốc)` | `data.extended.rationalThought` trên `profile`, ghi nguồn `birthName.given` |
| Ba Giai đoạn — Period Cycles | Giá trị `R(tháng sinh)`, `R(ngày sinh)`, `R(năm sinh)`; tuổi `0..E`, `E+1..E+27`, từ `E+28` | `data.extended.periodCycles` trên `cycles`/`profile` |
| Mức có mặt của các trục | Tám bộ 123, 456, 789, 147, 258, 369, 159, 357. Cả ba tần suất dương → `present`; cả ba bằng 0 → `absent`; còn lại → `partial` | `birthGridAxes` trên `birth`/`profile`; `birthNameGridAxes` trên `name`/`profile` |
| Tên thường gọi | Dùng bộ công thức tên, tính riêng trên tên được cung cấp | Đầu vào `extensions.currentName`; đầu ra `data.extended.currentName` trên `name`/`profile` |
| Biểu đồ kết hợp | Cộng tần suất theo từng số của biểu đồ ngày sinh và tên; ưu tiên currentName nếu có, nếu không dùng tên khai sinh | `data.extended.combinedGrid` trên `profile`, ghi `nameSource`, tần suất tên và các trục |
| Nhiều tên | Tính độc lập từng tên theo ID của bên gọi | `names` độc lập hoặc `extensions.nameSet` trên `profile` |
| Nhóm tên chọn ghép | Ghép đúng các phần full/given/middle/family đã chọn theo thứ tự, tính từ chữ gốc | `data.names.combinations` hoặc `data.extended.nameSet.combinations` |

Trục biểu đồ chỉ mô tả độ phủ chữ số, không gán nhãn tính cách hay luận giải. Tên thường gọi và nhóm ghép không tự thay tên khai sinh trong Trưởng thành, Cầu nối, Essence hay Tư duy gốc.

### Nhiều tên và nhóm ghép

Giữ nguyên ID và thứ tự do bên gọi chọn, không tự sinh mọi tổ hợp nickname. Hồ sơ dành ID `birth` cho tên khai sinh; `names` độc lập không tự thêm danh tính này. Phần `full` xếp given/middle/family; muốn thứ tự khác thì liệt kê rõ các phần không chồng nhau.

Nhóm ghép cộng **giá trị chữ gốc rồi mới rút gọn**, không cộng các kết quả đã rút. Ví dụ AIIII có tổng 37 → 1, A có tổng 1; ghép lại là 38 → 11 khi giữ master, không phải 1+1=2. Kết quả ghi `method=concatenateSelectedParts`, `reduction=wholeNameSum` và từng nguồn/chuỗi được chọn.

Khi có `birthDate`, mỗi tên trong `names` có Tư duy từ phần given riêng và ngày sinh. Nhóm dùng `D(V(chuỗi đã chọn) + ngày sinh nguyên gốc)`, ghi nguồn `selectedText`. Name set trong hồ sơ dùng ngày sinh của hồ sơ, không thay Tư duy khai sinh. Xem [giới hạn và quy tắc trùng nguồn](rules.vi.md#nhieu-ten).

### Hai Thử thách theo tên

Chỉ `name`/`profile` nhận `extensions.nameChallenges`, cần `indicators=true` và chọn rõ `nameOrder=givenMiddleFamily|familyMiddleGiven`. Tính riêng tên khai sinh và currentName nếu có; không tự mở rộng vào `names`/`nameSet` hay nhóm. Rust có hàm `calculate_name_challenges` cho một tên được truyền rõ ràng.

| Chỉ số | Công thức và quy ước | Trường trong `data.extended.nameChallenges.birthName` |
| --- | --- | --- |
| Thử thách Linh hồn | Trị tuyệt đối hiệu giá trị nguyên âm đầu–cuối trong tên đã xếp thứ tự; giữ 0 theo quy ước Numerologist đã dẫn nguồn | `soulUrge` |
| Thử thách Nhân cách | Trị tuyệt đối hiệu giá trị phụ âm đầu–cuối; hiệu 0 đổi thành 9 theo quy ước Roy Kirkland | `personality` |

Dùng cùng lựa chọn phân loại Y. Mỗi kết quả ghi chuỗi tên, thứ tự, Y, phương pháp, cách xử lý 0, hai chữ và giá trị, `difference` ban đầu, `value` cuối. Một chữ tương ứng duy nhất là cả hai đầu và cho hiệu 0. Không có chữ tương ứng thì `status=unavailable`, các đầu mút/hiệu/giá trị là null. Không gán master hoặc nợ nghiệp cho kiểu kết quả này. Xem [nguồn và phần chưa xác minh](formula-verification.vi.md).

<a id="doc-section-8"></a>

## Đọc đối tượng số và tái lập payload

Đối tượng số thông thường gồm `root` (tổng gốc), `steps` (các bước), `value` (kết quả), `isMaster` và `karmicDebt`. Nợ nghiệp chỉ được đánh dấu khi 13/14/16/19 xuất hiện trên chính đường rút gọn đó, không phải danh sách cố định hay tập hợp mọi chỉ số của các tên/ngày khác nhau. `reducedValue` phục vụ hiển thị: `value=11`, `isMaster=true`, `reducedValue=2` có thể trình bày là 11/2.

[Yêu cầu mẫu giả lập](examples/profile-request.json) và [phản hồi thật của CLI](examples/profile-response.json) minh họa hồ sơ đầy đủ, tên thường gọi riêng, một nhóm ghép và thứ tự chữ đầu–cuối. Không dùng hồ sơ người thật. Một số kết quả: Đường đời 6, Sứ mệnh 5, Trưởng thành 11 và số rút gọn hiển thị của Trưởng thành là 2. JSON ghi đủ các bước và thông tin phương pháp; không nên suy công thức chỉ từ tên nhãn đã dịch.

Ứng dụng cũ giữ nguyên yêu cầu để tiếp tục dùng payload cũ. Chức năng mới phải được gọi rõ; không âm thầm thay giá trị cũ bằng tên khác, nhóm ghép hay số rút gọn hiển thị.

<a id="doc-section-9"></a>

## Phạm vi chưa hỗ trợ

- Chưa có engine Chaldean, Vedic hay hệ khác ngoài Pythagorean.
- Chưa thêm Thử thách Sứ mệnh và “Năng lực trưởng thành” theo website đối chiếu vì chưa xác minh công thức. Chỉ số Trưởng thành/Maturity đã có theo công thức riêng nêu trên.
- Không có thư viện luận giải, tỷ lệ tính cách/nghề nghiệp, điểm tương hợp, đề xuất đổi tên tự động hay hệ số điện thoại/địa chỉ/tên doanh nghiệp.
- Kiểm chứng native hiện tại trên macOS ARM64. Nền tảng khác cần bản dựng và kiểm thử riêng; có binding ngôn ngữ không đồng nghĩa đã có mọi tệp native.
- SDK tính ngoại tuyến, xác định từ đầu vào; ứng dụng chọn cách hiển thị và luận giải.

<a id="doc-section-10"></a>

## Mã nguồn và căn cứ

Các công thức nằm trong [lõi cơ bản](../crates/numerology-core/src/lib.rs), [phần mở rộng](../crates/numerology-core/src/extensions.rs), [nhiều tên](../crates/numerology-core/src/names.rs) và [Thử thách theo tên](../crates/numerology-core/src/name_challenges.rs). Xem [quy tắc chính xác](rules.vi.md), [căn cứ công thức](formula-verification.vi.md), [kiểm thử lõi](../crates/numerology-core/tests) và [mẫu hợp đồng tương thích](../tests/fixtures).
