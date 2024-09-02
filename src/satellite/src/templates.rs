pub const SOCIAL_IMAGE_TEMPLATE: &str = r#"
<svg width="100%" height="100%" viewBox="0 0 288 152" version="1.1"
     xmlns="http://www.w3.org/2000/svg"
     xml:space="preserve">
	<rect x="0" y="0" width="288" height="151.2" style="fill:#ff00a9;"/>
    <path d="M52.844,28.23l16.095,18.538l24.911,-9.712l-14.963,21.17l16.096,18.538l-25.343,-5.455l-14.963,21.17l-0.7,-24.541l-25.344,-5.455l24.911,-9.712l-0.7,-24.541Z" style="fill:#ffed00;"/>
    <path d="M229.709,63.634c11.138,-9.979 23.885,-5.09 27.877,3.566c3.992,8.656 -0.773,21.079 -11.911,31.058c-8.035,7.607 -23.081,12.525 -35.024,15.07c-7.178,-9.878 -15.077,-23.595 -15.966,-34.624c-1.609,-14.868 3.155,-27.291 11.911,-31.059c8.756,-3.767 21.503,1.122 23.113,15.989Z" style="fill:#00ffeb;"/>
    <text x="0" y="100" font-size="48" text-anchor="start" fill="black"
          style="font-family: 'Open Sans', sans-serif; dominant-baseline: middle;
          overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
        {{title}}
    </text>
</svg>
"#;