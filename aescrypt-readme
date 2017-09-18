AES Crypt Linux Source Code
===========================

Contents

 1. License
 2. Installing
 3. AES Crypt Key File Format
 4. AES Crypt File Format Description

1. License

For all files except aes.c, aes.h, sha256.h, and sha256.c, the following
license applies:

This software is licensed as "freeware."  Permission to distribute
this software in source and binary forms is hereby granted without a
fee.  THIS SOFTWARE IS PROVIDED 'AS IS' AND WITHOUT ANY EXPRESSED OR
IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.
THE AUTHOR SHALL NOT BE HELD LIABLE FOR ANY DAMAGES RESULTING FROM
THE USE OF THIS SOFTWARE, EITHER DIRECTLY OR INDIRECTLY, INCLUDING,
BUT NOT LIMITED TO, LOSS OF DATA OR DATA BEING RENDERED INACCURATE.

The explicitly excluded files listed above are licensed under the
GNU Public License.  The license covering those files can be found here:
http://www.gnu.org/licenses/gpl-2.0.txt

2. Installing

Extract the source archive.  If you are reading this, you probably
already have.  Just in case you have not, extract the files using
a command like this:

   tar -xzf aescrypt-x.y.z.tgz

Replace x.y.z with the version information that is a part of the
filename.  This should create a directory with the same name, but
without .tgz.

Next change directories into aescrypt-x.y.z/src (or whatever the
name of the directory is that is created).

Type "make".  If this succeeds, you should have the executable files
compiled in the directory.  To install them, you can type
"make install" or manually copy the executable files wherever you
want them.  The two files of interest are "aescrypt"
and "aescrypt_keygen".

If the system tells you it could not find "make", you may not have
a C compiler and tools installed.  It should compile cleanly
and without warnings or errors.  If you get any, feel free to ask
about it on the discussion forum:

   http://forums.packetizer.com/viewforum.php?f=72

3. AES Crypt Key File Format

Starting with version 3.0.6 of AES Crypt for Linux, one may specify a key file
to use when encrypting or decrypting files.  This key file is stored in
Unicode format.  There is a utility in the source package called
aescrypt_keygen that will create a file for you in the required format.
However, you can use the Windows "Notepad" program to create the key file
or VIM on Linux.

The file is merely the password stored in UTF-16LE format.  The Windows
"Notepad" utility calls this "Unicode" in the "Encoding" drop-down list.

AES Crypt is smart enough to also handle both Little Endian and Big Endian
files.  However, AES Crypt does not process key files in UTF-8 format.

What is important is that the files MUST have the Byte Order Mark (BOM)
present so that AES Crypt will not misinterpret the order of octets in the
file.  Notepad adds this automatically.

If using VIM to create a key file, start vim for a new file like
"keyfile.key".  Then execute these commands in VIM:

    :set bomb
    :set fileencoding=utf-16le
    :wq!

As a third means of creating a file in the proper format, one can use the
iconv utility on Linux.  let's assume you have a password in a file in
UTF-8 format called "key-utf8.txt" and you wish to create "mykey.key" in
UTF-16 format.  This will do the trick:

    $ echo -ne '\xFF\xFE' >mykey.key
    $ iconv -f UTF-8 -t UTF-16LE key-utf8.txt >>mykey.key

Since all of these options are sometimes challenging for those not so
familiar with the command line or vim, we provide the aescrypt_keygen utility
to make life easier.  To use it, just type:

    $ aescrypt_keygen mykey.key

You can specify any name for the key file.  Just as with aescrypt, you
can also pass the password to the program via the -p parameter.  The example
above would prompt you for the password as AES Crypt normally would.

Once you have a key file generated, you can use a command like this to
encrypt files:

    $ aescrypt -e -k mykey.key somefile.txt

4. AESCrypt File Format Description

Items in quotes are a literal string.  Words outside of quotes
are a textual description of the contents.  Fixed-valued octets are
written in hexidecimal form (e.g., 0x01).

The AESCrypt version 2 file format is as follows.
      3 Octets - 'AES'
      1 Octet  - 0x02 (Version)
      1 Octet  - Reserved
     .... Start of repeating extension block section
      2 Octet  - Length in octets (in network byte order) of an extension
                 identifier and contents.  If 0x0000, then no further
                 extensions exist and the next octet is the start of the
                 Initialization Vector (IV).  Following an extension,
                 this length indicator would appear again to indicate
                 presence or absense of another extension and the size of
                 any such extension.
     nn Octets - Extension identifier.  This is either a URI or an
                 identifier defined by the AES developer community and
                 documented on the standard extensions page, either
                 of which is terminated by a single 0x00 octet.  All
                 extension identifiers are case sensitive.
                   Examples of URIs:
                      http://www.aescrypt.com/extensions/creator/
                      urn:oid:1.3.6.1.4.1.17090.55.14
                      urn:uuid:85519EA3-1DA6-45b9-9041-8CD368D8C086
                   Note:
                      A URI was used to allow anybody to define extension
                      types, though we should strive to define a standard
                      set of extensions.
                   Examples of standard extension identifiers:
                      CREATED-DATE
                      CREATED-BY
                 A special extension is defined that has no name, but is
                 merely a "container" for extensions to be added after the
                 AES file is initially created.  Such an extension avoids
                 the need to read and re-write the entire file in order to
                 add a small extension.  Software tools that create AES
                 files should insert a 128-octet "container" extension,
                 placing a 0x00 in the first octet of the extension
                 identifier field.  Developers may then insert extensions
                 into this "container" area and reduce the size of this
                 "container" as necessary.  If larger extensions are added
                 or the "container" area is filled entirely, then reading
                 and re-writing the entire file would be necessary to add
                 additional extensions.
     nn Octets - The contents of the extension
     .... End of repeating extension block section
     16 Octets - Initialization Vector (IV) used for encrypting the
                 IV and symmetric key that is actually used to encrypt
                 the bulk of the plaintext file.
     48 Octets - Encrypted IV and 256-bit AES key used to encrypt the
                 bulk of the file
                 16 octets - Initialization Vector
                 32 octets - encryption key
     32 Octets - HMAC
     nn Octets - Encrypted message (2^64 octets max)
      1 Octet  - File size modulo 16 in least significant bit positions
     32 Octets - HMAC

     Thus, the footprint of the file is at least 136 octets.

The AESCrypt version 1 file format is as follows.
      3 Octets - 'AES'
      1 Octet  - 0x01 (Version)
      1 Octet  - Reserved
     16 Octets - Initialization Vector (IV) used for encrypting the
                 IV and symmetric key that is actually used to encrypt
                 the bulk of the plaintext file.
     48 Octets - Encrypted IV and 256-bit AES key used to encrypt the
                 bulk of the file
                 16 octets - Initialization Vector
                 32 octets - encryption key
     32 Octets - HMAC
     nn Octets - Encrypted message (2^64 octets max)
      1 Octet  - File size modulo 16 in least significant bit positions
     32 Octets - HMAC

     Thus, the footprint of the file is at least 134 octets.

The AESCrypt version 0 file format is as follows.
      3 Octets - 'AES'
      1 Octet  - 0x00 (Version)
      1 Octet  - File size modulo 16 in least significant bit positions
     16 Octets - Initialization Vector (IV)
     nn Octets - Encrypted message (2^64 octets max)
     32 Octets - HMAC

     Thus, the footprint of the file is at least 53 octets.

