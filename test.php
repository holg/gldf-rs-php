<?php
$gldf_path = "tests/data/test.gldf";
$xml = gldf_to_xml($gldf_path);
$json = gldf_to_json($gldf_path);
$xml2 = xml_from_json($json);
echo $xml === $xml2;