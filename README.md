# gldf-rs-php
WIP php extenson and bindings for the gldf-rs rust library

create the library:

```
cargo build --release
```

copy the library to the PECL extension directory:
on MacOS wit Homebrew:
```
cp target/release/libgldf_rs_php.dylib /opt/homebrew/lib/php/pecl/20220829/libgldf_rs_php.so
```



```
<?php
$gldf_path = "tests/data/test.gldf";
$xml = gldf_to_xml($gldf_path);
$json = gldf_to_json($gldf_path);
$xml2 = xml_from_json($json);
echo $xml === $xml2;
```
=> 1
