# Hình ảnh README / README artwork

Dùng đúng logo đầy đủ PNG `full-logo-restored-draft.png` do chủ dự án chọn, gồm cả biểu tượng và chữ. Logo nằm ở vị trí nhận diện; bảng chữ số là minh họa chức năng Numerology, không dùng biểu tượng thương hiệu làm hình chức năng chính. `logo-7mlabs.png` là bản thu nhỏ giữ nguyên tỷ lệ, màu và nền trong suốt; không vẽ lại đường nét. Tệp gốc trong bộ nhận diện không bị sửa. Banner nhúng bản PNG này vào SVG để không phụ thuộc ảnh tải ngoài. Hình minh họa không phải giao diện sản phẩm đã phát hành.

Banner, sơ đồ và huy hiệu có bản sáng/tối. README dùng picture/source theo prefers-color-scheme, ảnh mặc định là bản sáng. Nền toàn trang GitHub do GitHub điều khiển, README không áp CSS để đổi nền trang. Tiếng Việt là bản chính, tiếng Anh có bố cục tương ứng.

SVG không dùng script, foreignObject hoặc phông/ảnh tải từ mạng; chỉ nhúng PNG logo cục bộ dưới dạng data URI. Huy hiệu mô tả thuộc tính dự án, không giả lập trạng thái CI hay chứng nhận độ chính xác.

The owner-selected restored full PNG logo, including its wordmark, is placed as the brand signature rather than the feature illustration. It is resized proportionally without redrawing or changing the original. Self-contained SVG banners embed that PNG. Each artwork has light/dark variants selected through picture/source and prefers-color-scheme, with a light fallback. GitHub controls the surrounding page theme. Static badges describe project properties rather than CI status or certified accuracy.

Theme-aware markup follows [GitHub's documented picture pattern](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/quickstart-for-writing-on-github).
