<a href="../README.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — logo đầy đủ"></a>

# Quy tắc tính Pythagorean 1.0

**Tiếng Việt** · [English](rules.md)

[Trang SDK](../README.md) · [Chức năng & công thức](features-and-formulas.vi.md) · **Quy tắc tính** · [Nguồn kiểm chứng](formula-verification.vi.md)

> SDK 1.1.0 bổ sung [đầu ra có lọc](payload-filtering.vi.md) với schema riêng; các bảng đường dẫn bên dưới mô tả payload đầy đủ. Công thức và quy tắc tính không đổi.

---

<details open>
<summary><strong>Trên trang này</strong></summary>

- [Đầu vào và tên](#doc-section-1)
- [Rút gọn và các chỉ số tên](#doc-section-2)
- [Ngày sinh và chỉ số kết hợp](#doc-section-3)
- [Chu kỳ, Đỉnh cao và Thử thách ngày sinh](#doc-section-4)
- [Essence](#doc-section-5)
- [Phạm vi và nguồn tham khảo](#doc-section-6)
- [Phần mở rộng tùy chọn — schema 1.1](#doc-section-7)
- [Nhiều tên và nhóm ghép được chọn rõ](#doc-section-8)
- [Thử thách theo chữ đầu–cuối của tên](#doc-section-9)

</details>

Mã quy tắc cơ bản: `pythagorean-7mlabs-1.0`. Mã này xác định các quy ước cụ thể bên dưới, không khẳng định mọi tác giả hoặc phần mềm Pythagorean đều dùng chung một tiêu chuẩn. Tài liệu này áp dụng cho quy tắc không đổi từ SDK 1.0.0 đến 1.1.0, gồm nhiều tên và phần mở rộng schema 1.1.

<a id="dau-vao"></a>
<a id="doc-section-1"></a>

## Đầu vào và tên

Ngày theo lịch Gregory, dạng `YYYY-MM-DD`, năm 0001–9999, có kiểm tra năm nhuận và số ngày hợp lệ. `cycles` và `profile` cần `referenceDate` rõ ràng. Không dùng ngầm ngày máy, múi giờ, trạng thái ngẫu nhiên hoặc dịch vụ mạng. Tên có `given` bắt buộc, `middle` và `family` tùy chọn; mỗi thành phần có thể chứa nhiều từ. Bên gọi quyết định cách gán thành phần phù hợp với tên Việt hay tên nước ngoài.

Chuẩn hóa tên: phân tách Unicode NFD, bỏ dấu kết hợp, đổi Đ/đ thành D, viết hoa ASCII. Khoảng trắng, gạch nối, nháy đơn thẳng/cong ngăn từ. Ký tự khác bị từ chối; bên gọi phải chuyển tự các hệ chữ chưa hỗ trợ, engine không âm thầm xóa chúng. Mỗi thành phần tối đa 1024 byte UTF-8; given phải có ít nhất một chữ được hỗ trợ.

Yêu cầu JSON tối đa 16 KiB. Từ chối trường lạ/trùng và kiểu dữ liệu sai. Không có họ hoặc đệm vẫn hợp lệ.

Bảng chữ: A/J/S=1, B/K/T=2, C/L/U=3, D/M/V=4, E/N/W=5, F/O/X=6, G/P/Y=7, H/Q/Z=8, I/R=9. AEIOU là nguyên âm, W luôn là phụ âm. `yRule=vowel` hoặc `consonant` áp dụng lựa chọn đó cho mọi Y. Mặc định `contextual` xem từng Y là nguyên âm khi không có AEIOU đứng liền trước/sau trong cùng từ. Đây là quy tắc cách viết, không phải phân tích phát âm; phản hồi dùng contextual Y có cảnh báo.

<a id="doc-section-2"></a>

## Rút gọn và các chỉ số tên

Rút gọn bằng cách cộng chữ số thập phân cho đến một chữ số; nếu `keepMasterNumbers=true` (mặc định), dừng thêm ở 11/22/33. Mỗi đối tượng số ghi tổng gốc, các bước, kết quả, cờ master và `karmicDebt` nếu 13/14/16/19 xuất hiện trên đúng đường rút gọn. Chấp nhận 0 cho nhóm không có chữ tương ứng, như không nguyên âm hoặc không họ; đây là kết quả số học của phạm vi rỗng.

Sứ mệnh cộng mọi chữ; Linh hồn cộng nguyên âm; Nhân cách cộng phụ âm. Cân bằng cộng chữ đầu của **mỗi từ** trong mọi thành phần. Chỉ số given/family chỉ dùng thành phần tương ứng. Tần suất đếm giá trị 1–9, số thiếu có tần suất 0, Đam mê tiềm ẩn giữ mọi giá trị đồng hạng cao nhất; Tiềm thức bằng 9 trừ số lượng giá trị thiếu. Cornerstone/Capstone là chữ đầu/cuối của given đã chuẩn hóa.

<a id="doc-section-3"></a>

## Ngày sinh và chỉ số kết hợp

Đường đời rút gọn tháng, ngày, năm riêng theo chính sách master, rồi rút gọn tổng ba kết quả theo cùng chính sách. Ngày sinh rút gọn ngày; Thái độ rút gọn ngày + tháng không giữ master. Biểu đồ ngày sinh đếm các chữ số khác 0 trong `YYYYMMDD` có đệm số 0. Trưởng thành rút gọn Đường đời + Sứ mệnh.

Hai Cầu nối là trị tuyệt đối hiệu của **giá trị được trả về**, lần lượt Đường đời/Sứ mệnh và Linh hồn/Nhân cách; không rút gọn tiếp, kể cả khi có master được giữ.

<a id="doc-section-4"></a>

## Chu kỳ, Đỉnh cao và Thử thách ngày sinh

Cho M/D/Y là tháng/ngày/năm sinh rút về một chữ số. P1=reduce(M+D), P2=reduce(D+Y), P3=reduce(P1.value+P2.value), P4=reduce(M+Y); các kết quả Đỉnh cao tuân chính sách master. Giai đoạn đầu có tuổi nguyên từ 0 đến E=`36 - Đường đời rút về một chữ số`, tính cả E. Hai giai đoạn sau lần lượt có chín tuổi nguyên: E+1..E+9 và E+10..E+18. Giai đoạn bốn từ E+19, không giới hạn trên (`endAge=null`). Đây không phải ngày chuyển giai đoạn cụ thể hay việc suy ra giai đoạn hiện tại.

Bốn Thử thách lần lượt là |M−D|, |D−Y|, hiệu tuyệt đối của hai Thử thách đầu và |M−Y|; không giữ master. Năm cá nhân rút M+D+năm tham chiếu rút một chữ số; Tháng cá nhân cộng tháng tham chiếu nguyên gốc vào Năm cá nhân; Ngày cá nhân cộng ngày tham chiếu nguyên gốc vào Tháng cá nhân.

Năm thế giới rút năm tham chiếu, Tháng thế giới cộng tháng, Ngày thế giới cộng ngày; mỗi bước rút về một chữ số, không giữ master.

Các chu kỳ năm dùng tháng 1–12, không đổi vào sinh nhật. Toàn bộ tính từ `referenceDate` được cung cấp. Xem [bảng công thức](features-and-formulas.vi.md) để đối chiếu từng trường đầu ra.

<a id="doc-section-5"></a>

## Essence

Mỗi thành phần tên ghép riêng các chữ đã chuẩn hóa. Một chữ chiếm số vị trí tuổi nguyên bằng giá trị của chữ, rồi chuyển chữ tiếp theo và lặp cả thành phần theo chu kỳ. Given/middle/family tạo ba dòng chuyển chữ độc lập. Thành phần tùy chọn rỗng đóng góp chữ null, giá trị 0.

Essence rút tổng giá trị của ba chữ đang hoạt động, theo chính sách master. Tuổi 0 bắt đầu ở chữ đầu. Có thể lấy 1–120 dòng nằm trong tuổi 0–120; `startAge` mặc định 0, `years` mặc định 100. `birthYearPlusAge` là nhãn cộng năm, không phải sinh nhật hay khoảng lịch. Không giả định tuổi thọ 99 năm.

<a id="doc-section-6"></a>

## Phạm vi và nguồn tham khảo

Các operation cơ bản: `name`, `birth`, `cycles`, `essence`, `profile`; bản 1.0.0 có thêm `names`. Profile kết hợp bốn phạm vi cơ bản và chỉ số dẫn xuất. Chưa triển khai Chaldean, văn bản luận giải, điểm tương hợp, nhóm tính cách hoặc hệ số điện thoại/địa chỉ/tên doanh nghiệp. Hệ khác cần bộ quy tắc riêng, không đổi ý nghĩa bộ hiện tại.

- Hans Decoz: [tự tính các chỉ số](https://www.worldnumerology.com/do-your-own-reading/).
- Hans Decoz: [Y là nguyên âm hay phụ âm](https://www.worldnumerology.com/numerology-articles/numerology-Y-vowel-consonant.html).
- [Ví dụ Đường đời có master ở thành phần](https://www.mynumerologycharts.com/life-path-calculator.html): 25/11/1975, 7 + 11 + 22 = 40 → 4; có mẫu kiểm thử số học tương ứng.
- [Quy ước Đỉnh cao](https://tarot.ac/numerology/cycles/pinnacles).

Các nguồn giúp xác định quy ước và điểm khác nhau. Tài liệu cùng mẫu số học cố định và kiểm thử bất biến là đặc tả có thể đối chiếu với code. SDK không đóng gói văn bản luận giải từ nguồn tham khảo.

<a id="phan-mo-rong"></a>
<a id="doc-section-7"></a>

## Phần mở rộng tùy chọn — schema 1.1

Quy tắc cơ bản, các trường Number và đầu ra schema 1.0 giữ nguyên. Bỏ `extensions` hoặc truyền null để giữ payload trước đó; truyền đối tượng sẽ chọn schema 1.1 và ghi các lựa chọn thực tế cùng `pythagorean-7mlabs-extensions-1.0`. Không có phần mở rộng nào tự bật theo ngày hiện tại hay nội dung tên. Toàn yêu cầu vẫn giới hạn 16 KiB, từ chối trường lạ/trùng và kiểu sai.

Hai công tắc mặc định true: `reducedValues` điều khiển trường số rút gọn, `indicators` điều khiển chỉ số mới trong `data.extended`. `currentName` dùng cùng cấu trúc tên, giới hạn, chuẩn hóa và Y với tên khai sinh. Chỉ nhận trên `name`/`profile` khi indicators=true; lỗi chuẩn hóa ghi đúng trường currentName. Contextual Y trong tên thường gọi có cảnh báo riêng. SDK không tự tách nickname ngăn bằng dấu phẩy hoặc chọn tên ưu tiên.

| Operation | Nội dung phù hợp trong data.extended |
| --- | --- |
| name | Các trục tên khai sinh; tên thường gọi và Thử thách tên nếu được yêu cầu |
| birth | Các trục ngày sinh |
| cycles | Ba Giai đoạn |
| essence | Không thêm nhóm chỉ số; vẫn có thể bật reducedValue |
| profile | Các phần phù hợp phía trên, Tư duy, biểu đồ kết hợp và name set nếu được yêu cầu |

### Số rút gọn để hiển thị

`reducedValue` là value rút về một chữ số không giữ master; 0 vẫn là 0. Không thay value hay thêm vào steps. Master được giữ có thể là `{root:33,value:33,steps:[33],isMaster:true,karmicDebt:null,reducedValue:6}`. Nếu tắt giữ master thì value=6, steps=[33,6], isMaster=false; phần hiển thị không phục hồi master đã bỏ. Số ghép 10 không phải master. Các Cầu nối vẫn dùng value gốc theo quy tắc, không dùng reducedValue để tính lại. Vẫn cộng toàn tên trước khi rút, không đổi sang rút từng thành phần trước.

### Tư duy — Rational Thought

Với profile: cộng giá trị mọi chữ trong `name.given` khai sinh, cộng ngày sinh nguyên gốc rồi rút không master. Ghi `nameSource=birthName.given`. Ví dụ THOMAS=22, ngày 3: 25 → 7. Không dùng currentName hay họ/đệm. Tham khảo [công thức Hans Decoz](https://www.worldnumerology.com/do-your-own-reading/); không cam kết tái hiện mọi chỉ số cùng tên của các website. Một hồ sơ trùng kết quả không đủ xác nhận toàn bộ cách chọn tên của website đó.

### Ba Giai đoạn — Period Cycles

Giá trị lần lượt là tháng, ngày, năm sinh rút theo `keepMasterNumbers`. Khoảng tuổi nguyên đầu từ 0 đến E=`36 - Đường đời rút một chữ số`, tính cả E; khoảng hai từ E+1 đến E+27, đủ 27 tuổi nguyên; khoảng ba từ E+28, `endAge=null`. `source` phân biệt birthMonth/birthDay/birthYear. Không phải khoảng lịch hay kết luận giai đoạn hiện tại; khác bốn Đỉnh cao.

Nguồn [Period Cycles](https://www.worldnumerology.com/do-your-own-reading/) mô tả giá trị và giai đoạn giữa 27 năm; cách tính tuổi nguyên bao gồm hai đầu được cố định rõ tại đây.

### Biểu đồ và các trục

Tám bộ số: 123, 456, 789, 147, 258, 369, 159, 357. Cả ba số có tần suất dương → `present`; cả ba 0 → `absent`; còn lại → `partial`. Không gán nhãn tâm lý hay luận giải. Trục tên khai sinh dùng tần suất toàn tên khai sinh; trục ngày sinh dùng chữ số Gregory.

Biểu đồ kết hợp của profile cộng tần suất theo từng giá trị của biểu đồ ngày sinh và tên được chọn. Có currentName thì dùng tần suất tên đó; không có thì dùng tên khai sinh. Ghi `nameSource`, giữ `nameFrequency`, trả tần suất tổng và các trục. Đây là phép cộng rõ ràng, không phải mô hình chấm điểm học máy.

### Tên thường gọi và phần chưa hỗ trợ

CurrentName dùng lại phép tính tên trên đúng tên được cung cấp; kết quả riêng, không tham gia Trưởng thành, Cầu nối, Essence hay Tư duy khai sinh. Bên gọi chọn các thành phần, có thể chỉ truyền given. Nguồn [Minor numbers](https://www.worldnumerology.com/do-your-own-reading/) dùng tên đầy đủ hiện dùng; given-only là phạm vi do bên gọi chọn, không mặc nhiên tương đương nguồn hoặc chỉ số Tiếp cận độc quyền.

Đã có hai Thử thách theo chữ đầu–cuối với cấu hình rõ ở phần sau. Chưa suy ra Thử thách Sứ mệnh, “Năng lực trưởng thành” theo website đối chiếu, danh sách nợ nghiệp tổng cố định, tỷ lệ tính cách/nghề nghiệp, điểm tương hợp, Chaldean hay văn bản luận giải. Cần xác minh công thức/phạm vi trước khi thêm.

<a id="nhieu-ten"></a>
<a id="doc-section-8"></a>

## Nhiều tên và nhóm ghép được chọn rõ

Mã quy tắc: `pythagorean-7mlabs-names-1.0`. Đây là phạm vi ghép do bên gọi chọn, không khẳng định mọi cách thực hành Pythagorean đều ghép toàn bộ nickname. Operation name và yêu cầu/kết quả cũ giữ nguyên.

### Chức năng độc lập

`operation=names` nhận `names:[{id,name}]`, tùy chọn `combinations:[{id,sources}]`, `birthDate`, `system`, `options`. Từng tên dùng cùng cấu trúc, chuẩn hóa, Y, master và lõi tính với một tên. ID phân biệt hoa/thường, duy nhất trên cả tên và nhóm, dài 1–64 ký tự ASCII theo `[A-Za-z0-9][A-Za-z0-9_.-]*`. Thứ tự đầu ra theo đầu vào. ID birth trong chế độ độc lập là ID thông thường, không phải nhãn danh tính đã xác thực.

Phản hồi schema 1.1 ghi mã quy tắc cơ bản, `namesRulesVersion` và `data.names` chứa rulesVersion, names riêng, combinations. Mỗi tên có ID, nguồn `input`, toàn bộ kết quả tên; nếu có birthDate thì Tư duy dùng given riêng của tên đó. Không thay số của tên khác. Không có birthDate thì không trả Tư duy; có ngày sai thì báo lỗi. Luôn có reducedValue trong kết quả số của names độc lập, không sửa value/steps. Yêu cầu vẫn tối đa 16 KiB.

### Chọn nguồn và rút gọn

Mỗi nguồn nhóm gồm `nameId`, `part` tùy chọn: full mặc định, given, middle hoặc family. Full ghép các thành phần không rỗng theo given/middle/family; cần thứ tự khác thì liệt kê các phần không chồng nhau. Ghép các thành phần/nguồn đã chuẩn hóa bằng khoảng trắng, giữ ranh giới từ cho Y. Kết quả ghi ID, phần thực tế và chuỗi của từng nguồn theo thứ tự. Nhóm không được tham chiếu nhóm khác, không có đệ quy hoặc tự sinh hoán vị.

Chuỗi ghép được tính bằng cùng lõi tên như một given tổng hợp, middle/family rỗng. Kết quả có `method=concatenateSelectedParts`, `reduction=wholeNameSum`, chuỗi chuẩn hóa, tổng gốc và đường rút gọn. Không cộng số đã rút: AIIII tổng 37 → 1 và A tổng 1, ghép thành 38 → 11 nếu giữ master, không phải 1+1=2. Tắt master thì 38 → 11 → 2.

Có birthDate thì Tư duy nhóm cộng ngày sinh nguyên gốc vào tổng chữ của toàn chuỗi được chọn, ghi `nameSource=selectedText`. Đây là phép tính theo phạm vi nhóm, không tự thay Tư duy khai sinh. Cornerstone/Capstone và Cân bằng của nhóm cũng mô tả chuỗi tổng hợp theo thứ tự, không đổi danh tính tên khai sinh.

### Trùng lặp và giới hạn

Giữ mọi tên riêng kể cả khi chuẩn hóa làm các tên giống nhau. Nhóm từ chối cùng chuỗi chuẩn hóa được chọn lặp lại hoặc các phần chọn lặp/chồng của cùng tên chuẩn hóa, kể cả qua hai ID khác nhau. Báo lỗi, không âm thầm loại trùng. Được chọn given/family không chồng của một tên. Chọn phần tùy chọn rỗng → EMPTY_NAME_PART; ID không tồn tại → UNKNOWN_NAME_ID; ID sai, trùng ID hoặc trùng nguồn có lỗi ghi trường cụ thể.

Giới hạn: 1–32 tên tổng cộng, 0–32 nhóm, 2–32 nguồn mỗi nhóm, tối đa 4096 byte UTF-8 cho chuỗi ghép chuẩn hóa. Mỗi thành phần tên đầu vào vẫn tối đa 1024 byte UTF-8. Không tự ghép tên gần giống, chọn biệt danh, gán trọng số hoặc luận giải ngữ nghĩa.

### Trong hồ sơ và tương thích

Profile có thể truyền `extensions.nameSet:{names,combinations}`. Tự thêm tên khai sinh đã chuẩn hóa với ID `birth`, nguồn `profile.birthName`; nhận tối đa 31 tên bổ sung. Danh sách bỏ qua mặc định rỗng; nameSet={} trả riêng mục birth. Không cho dùng lại ID birth. Đầu ra ở `data.extended.nameSet`, ghi phiên bản quy tắc tên trong metadata mở rộng. Cần indicators=true và operation=profile; cách dùng khác báo INVALID_EXTENSION, không âm thầm bỏ qua.

Name set tôn trọng reducedValues của profile. Các trường hồ sơ cũ — Tư duy khai sinh, Trưởng thành, Cầu nối, Essence, currentName và nguồn biểu đồ kết hợp — giữ nguyên. Nhóm không tự trở thành currentName hay đầu vào tính lại hồ sơ gốc.

Rust có `NamesRequest`/`calculate_names`, `NameSet`/`calculate_with_name_set` riêng. Cấu trúc công khai Request, Extensions, Number và các hàm cũ được giữ. Mọi binding dùng chung thực thi JSON, không cần mạng. TypeScript giữ miền cũ của Request/Operation; NamesRequest cho đầu vào mới, AnyRequest/SupportedOperation bao quát cả hai; Result generic cũ vẫn hợp lệ, NamesResult mô tả kết quả nhiều tên.

<a id="doc-section-9"></a>

## Thử thách theo chữ đầu–cuối của tên

Mặc định `extensions.nameChallenges` không có hoặc null. Đối tượng khác null bắt buộc `nameOrder=givenMiddleFamily|familyMiddleGiven`, operation name/profile, indicators=true. Tính riêng từ `data.name.normalizedName` và currentName nếu có, không đổi dữ liệu ngày sinh, tổng tên, name set hoặc biểu đồ kết hợp. Bỏ phần họ/đệm rỗng, giữ ranh giới từ và Y đang cấu hình.

`data.extended.nameChallenges` chứa birthName và currentName nếu có. Mỗi mục ghi `rulesVersion=pythagorean-7mlabs-name-challenges-1.0`, orderedText, nameOrder, yRule, phương pháp, cách xử lý 0 và hai phép tính soulUrge/personality. Hai chữ đầu–cuối ghi giá trị Pythagorean 1–9; difference là trị tuyệt đối hiệu. Linh hồn giữ 0 theo nguồn Blair Andrews/Numerologist; Nhân cách đổi hiệu 0 thành 9 theo Roy Kirkland. keepMasterNumbers/reducedValues không đổi các hiệu này. Đây không phải đối tượng CalculatedNumber, không thêm nhãn master/nợ nghiệp.

Cần ít nhất một chữ đúng loại: một chữ là cả hai đầu, cho hiệu 0; không có chữ đúng loại trả status=unavailable, reason=noMatchingLetters, first/last/difference/value=null. Có chữ thì status=available. Chỉ thêm nameChallengesRulesVersion trong metadata khi được bật. Các cấu trúc Rust và yêu cầu JSON cũ giữ nguyên. Hàm `calculate_name_challenges(&Name,&Options,&NameChallengeOptions)` tính một tên/biệt danh được cung cấp rõ; loại cấu hình mới đứng riêng, không có mặc định đoán thứ tự tên theo văn hóa.

Xem [căn cứ công thức và hai phương pháp chưa xác minh](formula-verification.vi.md).
