# rust-victoria-benchmark
Send from Rust to Victoria as fast as you can


Examples
Write a point to the database mydb with a timestamp in seconds

curl -d "disk_free free_space=442221834240i,disk_type=\"SSD\" $(date +%s%N | cut -b1-13)" -X POST 'http://localhost:8428/write'

curl -d "disk_free value=12i" -X POST 'http://localhost:8428/write'


curl -G 'http://localhost:8428/api/v1/export' -d 'match={__name__=~"disk_free.*"}'


curl -d '00000000-0000-0000-0000-000000000000,objectId=00000000-0000-0000-0000-0000000000dd value=260.7517906281873,quality_value=0u,quality_reason=0u,limit=0,flags=0u,time_valid=False,day_no=0i,time=0i' -X POST 'http://localhost:8428/write'


curl -G 'http://localhost:8428/api/v1/export' -d 'match={__name__=~"00000000-0000-0000-0000-000000000000"}'