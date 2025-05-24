#!/bin/sh

input="./sample.d/input.zip"

geninput(){

	echo generating input zip file...

	mkdir -p sample.d

	dat0=$(
		jq -c -n '{ name: "fuji", height: 3.776 }' |
			xxd -ps |
			tr -d '\n'
	)

	dat1=$(
		jq -c -n '{ name: "takao", height: 0.599 }' |
			xxd -ps |
			tr -d '\n'
	)

	dat2=$(
		jq -c -n '{ name: "FUJI", height: 3.776 }' |
			xxd -ps |
			tr -d '\n'
	)

	dat3=$(
		jq -c -n '{ name: "TAKAO", height: 0.599 }' |
			xxd -ps |
			tr -d '\n'
	)

	jq \
		-n \
		--arg dat0 "${dat0}" \
		--arg dat1 "${dat1}" \
		-c '[
		{
			meta: {
				filename: "/path/to/filename/j0.json",
				comment: "",
				modified: 1747708241,
				compression: "store",
				isDir: false
			},
			data: $dat0
		},
		{
			meta: {
				filename: "/path/to/filename/j1.json",
				comment: "",
				modified: 1747708241,
				compression: "store",
				isDir: false
			},
			data: $dat1
		}
	]' |
		xxd -ps |
		tr -d '\n' |
		python3 \
			-m asn1tools \
			convert \
			-i jer \
			-o der \
			./zipitem.asn \
			ZipItems \
			- |
			xxd -r -ps |
			cat > ./sample.d/z0.jsons.zipitems.asn1.dat

	jq \
		-n \
		--arg dat2 "${dat0}" \
		--arg dat3 "${dat1}" \
		-c '[
		{
			meta: {
				filename: "/path/to/filename/j2.json",
				comment: "",
				modified: 1747708241,
				compression: "store",
				isDir: false
			},
			data: $dat2
		},
		{
			meta: {
				filename: "/path/to/filename/j3.json",
				comment: "",
				modified: 1747708241,
				compression: "store",
				isDir: false
			},
			data: $dat3
		}
	]' |
		xxd -ps |
		tr -d '\n' |
		python3 \
			-m asn1tools \
			convert \
			-i jer \
			-o der \
			./zipitem.asn \
			ZipItems \
			- |
			xxd -r -ps |
			cat > ./sample.d/z1.jsons.zipitems.asn1.dat

	ls ./sample.d/z*.asn1.dat |
		zip \
			-@ \
			-T \
			-v \
			-o \
			"${input}"

}

test -f "${input}" || geninput

wazero run \
	-env ENV_INPUT_ZIP_FILENAME=/guest.d/input.zip \
	-mount "./sample.d:/guest.d:ro" \
	./rs-zip2asn2data2concat.wasm |
	jq -c
