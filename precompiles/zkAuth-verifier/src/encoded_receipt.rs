// Copyright 2024 Moonbeam Foundation.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

// TODO: only used for testing purposes
//
// This is an encoded receipt of a risc0 guest program that performs a single validation over a
// mocked JWT.
//
// It can be verified using the following ImageID:
//
// ImageID = [
//	1923256869, 654795233, 2887859926, 1709721587, 1196091263, 3916749566, 1248329059, 610202488
// ]

use sp_std::vec::Vec;

pub fn encoded_example_receipt() -> Vec<u8> {
	Vec::from([
		2, 128, 2, 25, 116, 221, 142, 137, 191, 250, 34, 170, 169, 128, 34, 127, 166, 231, 34, 210,
		70, 78, 177, 139, 1, 185, 75, 107, 191, 186, 232, 177, 33, 186, 83, 47, 185, 163, 124, 86,
		179, 151, 229, 213, 193, 66, 14, 252, 150, 23, 225, 144, 162, 193, 177, 25, 248, 161, 208,
		12, 145, 63, 25, 96, 103, 61, 63, 28, 185, 136, 67, 220, 1, 196, 236, 250, 8, 181, 55, 251,
		57, 146, 71, 161, 138, 211, 247, 110, 105, 90, 78, 77, 112, 180, 81, 62, 158, 110, 66, 32,
		113, 100, 222, 132, 63, 144, 101, 238, 168, 104, 49, 72, 53, 204, 165, 248, 130, 63, 23,
		157, 237, 115, 172, 176, 202, 248, 50, 166, 188, 42, 35, 17, 11, 51, 12, 178, 121, 16, 62,
		133, 2, 223, 115, 202, 25, 196, 23, 156, 128, 174, 91, 185, 227, 218, 18, 235, 158, 134,
		83, 123, 43, 250, 78, 16, 87, 92, 103, 72, 29, 214, 174, 70, 75, 201, 163, 28, 153, 57,
		120, 216, 146, 218, 105, 63, 152, 171, 228, 196, 31, 100, 225, 77, 119, 152, 239, 18, 172,
		150, 220, 80, 96, 94, 67, 252, 173, 119, 63, 244, 180, 198, 187, 13, 96, 12, 165, 150, 241,
		196, 37, 10, 170, 253, 165, 208, 128, 57, 45, 22, 112, 121, 28, 167, 40, 48, 95, 189, 233,
		91, 175, 235, 147, 195, 128, 4, 160, 157, 87, 129, 232, 179, 128, 115, 168, 102, 238, 210,
		121, 87, 232, 0, 0, 152, 236, 128, 1, 180, 189, 196, 141, 2, 234, 194, 182, 210, 13, 173,
		202, 134, 192, 15, 148, 159, 157, 167, 11, 246, 187, 206, 253, 14, 227, 149, 212, 206, 9,
		133, 216, 160, 110, 240, 169, 185, 187, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 1, 0, 180, 4, 29, 2, 0, 0, 101, 121, 74, 48, 101, 88, 65, 105, 79, 105,
		74, 75, 86, 49, 81, 105, 76, 67, 74, 104, 98, 71, 99, 105, 79, 105, 74, 83, 85, 122, 73,
		49, 78, 105, 73, 115, 73, 109, 116, 112, 90, 67, 73, 54, 73, 106, 103, 51, 79, 84, 74, 108,
		78, 50, 77, 121, 89, 84, 74, 105, 78, 50, 77, 120, 89, 87, 73, 53, 77, 106, 82, 108, 77,
		84, 85, 52, 89, 84, 82, 108, 89, 122, 82, 106, 90, 106, 85, 120, 73, 110, 48, 46, 101, 121,
		74, 108, 98, 87, 70, 112, 98, 67, 73, 54, 73, 110, 82, 108, 99, 51, 82, 65, 90, 87, 49,
		104, 97, 87, 119, 117, 89, 50, 57, 116, 73, 105, 119, 105, 98, 109, 57, 117, 89, 50, 85,
		105, 79, 105, 73, 119, 101, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119,
		77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77,
		68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 105, 102, 81,
		46, 84, 80, 85, 114, 109, 83, 116, 119, 89, 50, 105, 117, 113, 77, 76, 88, 110, 51, 87,
		118, 112, 105, 74, 89, 49, 87, 45, 98, 98, 114, 85, 49, 50, 87, 71, 117, 118, 48, 110, 75,
		57, 78, 74, 54, 81, 48, 98, 84, 56, 68, 95, 65, 103, 115, 56, 113, 106, 56, 76, 80, 79, 71,
		71, 69, 49, 67, 100, 72, 110, 50, 105, 115, 66, 99, 72, 103, 83, 120, 97, 69, 98, 78, 98,
		87, 56, 80, 122, 48, 102, 86, 87, 112, 70, 105, 101, 104, 106, 56, 66, 119, 114, 67, 52,
		55, 82, 108, 100, 53, 100, 119, 97, 122, 115, 120, 103, 104, 70, 56, 52, 68, 51, 113, 50,
		83, 111, 53, 90, 66, 81, 115, 108, 87, 113, 113, 49, 80, 82, 71, 69, 70, 75, 102, 120, 52,
		65, 79, 103, 110, 83, 51, 55, 53, 111, 75, 105, 50, 106, 65, 90, 51, 106, 78, 95, 53, 56,
		85, 78, 100, 103, 116, 85, 85, 100, 70, 104, 117, 79, 71, 72, 118, 71, 98, 87, 110, 114,
		95, 102, 69, 87, 73, 98, 114, 69, 99, 102, 78, 70, 73, 87, 97, 104, 110, 103, 81, 50, 100,
		98, 85, 45, 115, 83, 78, 90, 70, 90, 53, 76, 51, 76, 52, 54, 98, 88, 85, 107, 66, 108, 98,
		71, 71, 78, 122, 116, 114, 54, 79, 105, 65, 72, 85, 119, 120, 113, 72, 50, 65, 48, 50, 104,
		49, 69, 99, 101, 85, 111, 108, 50, 109, 54, 95, 71, 84, 118, 80, 102, 100, 88, 75, 122,
		100, 48, 90, 51, 52, 67, 74, 78, 87, 95, 108, 111, 65, 69, 104, 101, 72, 54, 57, 104, 107,
		109, 107, 71, 80, 98, 116, 51, 116, 97, 95, 88, 65, 70, 87, 82, 72, 103, 109, 86, 78, 55,
		103, 70, 106, 69, 114, 82, 109, 80, 105, 66, 56, 49, 56, 89, 103, 65, 70, 66, 66, 73, 117,
		104, 90, 110, 106, 118, 71, 109, 67, 53, 81, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0,
		3, 0, 0, 0, 0, 0, 177, 158, 148, 199, 9, 232, 199, 167, 255, 2, 168, 139, 240, 189, 2, 173,
		254, 225, 194, 7, 152, 247, 174, 196, 6, 196, 171, 223, 158, 9, 141, 241, 146, 220, 10,
		170, 237, 133, 231, 5, 180, 4, 29, 2, 0, 0, 101, 121, 74, 48, 101, 88, 65, 105, 79, 105,
		74, 75, 86, 49, 81, 105, 76, 67, 74, 104, 98, 71, 99, 105, 79, 105, 74, 83, 85, 122, 73,
		49, 78, 105, 73, 115, 73, 109, 116, 112, 90, 67, 73, 54, 73, 106, 103, 51, 79, 84, 74, 108,
		78, 50, 77, 121, 89, 84, 74, 105, 78, 50, 77, 120, 89, 87, 73, 53, 77, 106, 82, 108, 77,
		84, 85, 52, 89, 84, 82, 108, 89, 122, 82, 106, 90, 106, 85, 120, 73, 110, 48, 46, 101, 121,
		74, 108, 98, 87, 70, 112, 98, 67, 73, 54, 73, 110, 82, 108, 99, 51, 82, 65, 90, 87, 49,
		104, 97, 87, 119, 117, 89, 50, 57, 116, 73, 105, 119, 105, 98, 109, 57, 117, 89, 50, 85,
		105, 79, 105, 73, 119, 101, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119,
		77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77,
		68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 105, 102, 81,
		46, 84, 80, 85, 114, 109, 83, 116, 119, 89, 50, 105, 117, 113, 77, 76, 88, 110, 51, 87,
		118, 112, 105, 74, 89, 49, 87, 45, 98, 98, 114, 85, 49, 50, 87, 71, 117, 118, 48, 110, 75,
		57, 78, 74, 54, 81, 48, 98, 84, 56, 68, 95, 65, 103, 115, 56, 113, 106, 56, 76, 80, 79, 71,
		71, 69, 49, 67, 100, 72, 110, 50, 105, 115, 66, 99, 72, 103, 83, 120, 97, 69, 98, 78, 98,
		87, 56, 80, 122, 48, 102, 86, 87, 112, 70, 105, 101, 104, 106, 56, 66, 119, 114, 67, 52,
		55, 82, 108, 100, 53, 100, 119, 97, 122, 115, 120, 103, 104, 70, 56, 52, 68, 51, 113, 50,
		83, 111, 53, 90, 66, 81, 115, 108, 87, 113, 113, 49, 80, 82, 71, 69, 70, 75, 102, 120, 52,
		65, 79, 103, 110, 83, 51, 55, 53, 111, 75, 105, 50, 106, 65, 90, 51, 106, 78, 95, 53, 56,
		85, 78, 100, 103, 116, 85, 85, 100, 70, 104, 117, 79, 71, 72, 118, 71, 98, 87, 110, 114,
		95, 102, 69, 87, 73, 98, 114, 69, 99, 102, 78, 70, 73, 87, 97, 104, 110, 103, 81, 50, 100,
		98, 85, 45, 115, 83, 78, 90, 70, 90, 53, 76, 51, 76, 52, 54, 98, 88, 85, 107, 66, 108, 98,
		71, 71, 78, 122, 116, 114, 54, 79, 105, 65, 72, 85, 119, 120, 113, 72, 50, 65, 48, 50, 104,
		49, 69, 99, 101, 85, 111, 108, 50, 109, 54, 95, 71, 84, 118, 80, 102, 100, 88, 75, 122,
		100, 48, 90, 51, 52, 67, 74, 78, 87, 95, 108, 111, 65, 69, 104, 101, 72, 54, 57, 104, 107,
		109, 107, 71, 80, 98, 116, 51, 116, 97, 95, 88, 65, 70, 87, 82, 72, 103, 109, 86, 78, 55,
		103, 70, 106, 69, 114, 82, 109, 80, 105, 66, 56, 49, 56, 89, 103, 65, 70, 66, 66, 73, 117,
		104, 90, 110, 106, 118, 71, 109, 67, 53, 81, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0,
		3, 0, 0, 0, 177, 158, 148, 199, 9, 232, 199, 167, 255, 2, 168, 139, 240, 189, 2, 173, 254,
		225, 194, 7, 152, 247, 174, 196, 6, 196, 171, 223, 158, 9, 141, 241, 146, 220, 10, 170,
		237, 133, 231, 5,
	])
}
