<a href="../README.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — logo đầy đủ"></a>

# Kiểm chứng công thức Thử thách theo tên

**Tiếng Việt** · [English](formula-verification.md)

[Trang SDK](../README.md) · [Chức năng & công thức](features-and-formulas.vi.md) · [Quy tắc tính](rules.vi.md) · **Nguồn kiểm chứng**

---

Ngày nghiên cứu: 11/11/2026. Tài liệu ghi quy ước số học được nguồn mô tả và phạm vi đã triển khai. Engine xuất rõ cách tính, không kèm văn bản luận giải.

| Chỉ số được yêu cầu | Căn cứ | Quyết định |
| --- | --- | --- |
| Thử thách Linh hồn — Soul Urge Challenge | Blair Andrews/Numerologist dùng nguyên âm đầu–cuối của tên khai sinh đầy đủ, lấy giá trị lớn trừ nhỏ, kết quả 0–8. | Đã triển khai hiệu tuyệt đối, giữ 0. |
| Thử thách Nhân cách — Personality Challenge | Phỏng vấn Roy Kirkland/Numerologist dùng phụ âm đầu–cuối. Dave Jones ra 3; Steve Jones có hiệu 0 và quy ước của tác giả đổi thành 9. | Đã triển khai đúng quy ước 0 → 9 này, ghi cả hiệu gốc và kết quả. |
| Thử thách Sứ mệnh — Expression Challenge | Eagle Spirit Ministry mô tả **cộng** hai Thử thách theo tên rồi rút một chữ số. Chưa chứng minh đó là cách của website đối chiếu: website trả hai Thử thách 2 và 3 nhưng Thử thách Sứ mệnh 1; phép cộng cho 5. | Chưa thêm. Không suy ra phép trừ từ một mẫu, không coi công thức cộng là tương đương website. |
| Năng lực trưởng thành | Website đối chiếu có mô tả ý nghĩa nhưng chưa tìm được công thức cụ thể. Hans Decoz có Cầu nối Đường đời–Sứ mệnh, có mô tả giá trị một chữ số trong báo cáo. Hiệu số sau rút gọn có thể khớp một mẫu nhưng chưa chứng minh nhãn tiếng Việt là thuật toán đó. | Chưa thêm công thức hay tên gọi tương đương suy đoán. Giữ nguyên Trưởng thành/Maturity và các Cầu nối dùng giá trị thô. |

## Nguồn gốc

1. [Blair Andrews: cách tính Soul Urge Challenge](https://numerologist.com/personal-growth-and-success/personal-development-guide/learn-secrets-self-attunment), phần “Working With Your Soul Urge Challenge”.
2. [Tài liệu Change Your Life của Numerologist.com](https://s3.amazonaws.com/training-ebooks/change-your-life.pdf), trang PDF 29: David Edward Hanson, nguyên âm đầu A=1, cuối O=6, hiệu 5.
3. [Phỏng vấn Roy Kirkland, bản chép lời](https://numerologist.com/numerology/episode-43-your-personality-number-reveals-the-secrets-of-what-other-people-think-about-you-with-roy-kirkland), phần trả lời cách tìm Personality và Challenge; ví dụ Dave Jones/Steve Jones xác định phép trừ và quy tắc đổi 0.
4. [Eagle Spirit Ministry: bài Numerology thứ 22](https://eaglespiritministry.com/pd/sdc/sdc22a.htm), phần Expression Challenges. Câu chữ về Thử thách đầu–cuối gần đó kém rõ, không dùng để quyết định chính sách số 0 của SDK.
5. [Hans Decoz: Bridge Numbers](https://www.worldnumerology.com/numerology-bridge-numbers/) mô tả hiệu các chỉ số; [báo cáo mẫu chính thức](https://www.worldnumerology.com/member-report-samples/ultimate-numerology-report.pdf), trang PDF 12 có cách diễn đạt một chữ số cho Cầu nối Linh hồn–Nhân cách. Không đủ xác định tên tiếng Việt hoặc mọi trường hợp master của Năng lực trưởng thành trên website đối chiếu.
6. [Tổng quan của Tra Cứu Thần Số Học](https://tracuuthansohoc.com/than-so-hoc-la-gi/) có nhãn và ý nghĩa Năng lực trưởng thành; mô tả ý nghĩa chưa phải đặc tả tính toán.

## Phạm vi và kiểm thử

Thứ tự thành phần tên phải được chọn rõ: đổi given/middle/family thành family/middle/given có thể đổi chữ đầu–cuối dù tổng tên không đổi. Tái sử dụng chuẩn hóa và lựa chọn phân loại Y của SDK. Nguồn không quy định phát âm Y cho mọi ngôn ngữ; SDK nêu rõ quy tắc cách viết và cho chọn nguyên âm/phụ âm trực tiếp.

Phần mới chỉ có khi bật extensions.nameChallenges trên name/profile, hoặc gọi hàm Rust riêng. Biệt danh có thể truyền độc lập. Danh sách tên và nhóm ghép cũ giữ nguyên, không tự gán thành Thử thách tên khai sinh. Không đóng gói văn bản nguồn hoặc lời luận giải.

Kiểm thử bao gồm ví dụ công bố, hiệu 0, không có chữ đúng loại, một chữ, hai thứ tự tên, chuẩn hóa/chế độ Y, không phụ thuộc công tắc master, tách tên thường gọi, dùng chung với name set, cấu hình sai, tính tương đương đầu ra cũ và bảng tổ hợp chữ đầu–cuối cùng loại. Các binding đối chiếu cùng bộ ca JSON; mẫu sinh từ CLI kiểm tra sự nhất quán giữa binding, còn ví dụ Rust độc lập kiểm tra số học.
