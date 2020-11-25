
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![],
  meta_content_scope: vec![],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)(\\\\)?\\b(\n  AMQPConnection | AMQPExchange | AMQPQueue | APCIterator | AppendIterator | ArrayAccess | ArrayIterator | ArrayObject |\n  BadFunctionCallException | BadMethodCallException | CachingIterator | Collator | Countable | DOMAttr | DOMCharacterData | DOMComment |\n  DOMDocument | DOMDocumentFragment | DOMElement | DOMEntityReference | DOMImplementation | DOMNamedNodeMap | DOMNode | DOMNodelist |\n  DOMProcessingInstruction | DOMText | DOMXPath | DateInterval | DatePeriod | DateTime | DateTimeZone | DirectoryIterator |\n  DomAttribute | DomDocument | DomDocumentType | DomElement | DomNode | DomProcessingInstruction | DomXsltStylesheet | DomainException |\n  EmptyIterator | ErrorException | Exception | FilesystemIterator | FilterIterator | GlobIterator | Gmagick | GmagickDraw |\n  GmagickPixel | HaruAnnotation | HaruDestination | HaruDoc | HaruEncoder | HaruFont | HaruImage | HaruOutline |\n  HaruPage | HttpDeflateStream | HttpInflateStream | HttpMessage | HttpQueryString | HttpRequest | HttpRequestPool | HttpResponse |\n  Imagick | ImagickDraw | ImagickPixel | ImagickPixelIterator | InfiniteIterator | IntlDateFormatter | InvalidArgumentException | Iterator |\n  IteratorAggregate | IteratorIterator | JsonException | JsonSerializable | KTaglib_ID3v2_AttachedPictureFrame | KTaglib_ID3v2_Frame | KTaglib_ID3v2_Tag | KTaglib_MPEG_AudioProperties |\n  KTaglib_MPEG_File | KTaglib_Tag | LengthException | LimitIterator | Locale | LogicException | Memcache | Memcached |\n  MessageFormatter | Mongo | MongoBinData | MongoCode | MongoCollection | MongoCursor | MongoDB | MongoDBRef |\n  MongoDate | MongoGridFS | MongoGridFSCursor | MongoGridFSFile | MongoGridfsFile | MongoId | MongoInt32 | MongoInt64 |\n  MongoRegex | MongoTimestamp | MultipleIterator | NoRewindIterator | Normalizer | NumberFormatter | OCI-Collection | OCI-Lob |\n  OutOfBoundsException | OutOfRangeException | OuterIterator | OverflowException | PDO | PDOStatement | ParentIterator | Phar |\n  PharData | PharFileInfo | RRDCreator | RRDGraph | RRDUpdater | RangeException | RecursiveArrayIterator | RecursiveCachingIterator |\n  RecursiveDirectoryIterator | RecursiveFilterIterator | RecursiveIterator | RecursiveIteratorIterator | RecursiveRegexIterator | RecursiveTreeIterator | Reflection | ReflectionClass |\n  ReflectionExtension | ReflectionFunction | ReflectionFunctionAbstract | ReflectionMethod | ReflectionObject | ReflectionParameter | ReflectionProperty | Reflector |\n  RegexIterator | ResourceBundle | RuntimeException | SAMConnection | SAMMessage | SCA | SCA_LocalProxy | SCA_SoapProxy |\n  SDO_DAS_ChangeSummary | SDO_DAS_DataFactory | SDO_DAS_DataObject | SDO_DAS_Relational | SDO_DAS_Setting | SDO_DAS_XML | SDO_DAS_XML_Document | SDO_DataFactory |\n  SDO_DataObject | SDO_Exception | SDO_List | SDO_Model_Property | SDO_Model_ReflectionDataObject | SDO_Model_Type | SDO_Sequence | SNMP |\n  SQLite3 | SQLite3Result | SQLite3Stmt | SQLiteDatabase | SQLiteResult | SQLiteUnbuffered | SVM | SVMModel |\n  SeekableIterator | Serializable | SimpleXMLElement | SimpleXMLIterator | SoapClient | SoapFault | SoapHeader | SoapParam |\n  SoapServer | SoapVar | SphinxClient | SplBool | SplDoublyLinkedList | SplEnum | SplFileInfo | SplFileObject |\n  SplFixedArray | SplFloat | SplHeap | SplInt | SplMaxHeap | SplMinHeap | SplObjectStorage | SplObserver |\n  SplPriorityQueue | SplQueue | SplStack | SplString | SplSubject | SplTempFileObject | Swish | SwishResult |\n  SwishResults | SwishSearch | TokyoTyrant | TokyoTyrantQuery | TokyoTyrantTable | Transliterator | Traversable | UnderflowException |\n  UnexpectedValueException | V8Js | V8JsException | XMLReader | XMLWriter | XSLTProcessor | ZipArchive | finfo |\n  mysqli | mysqli_driver | mysqli_result | mysqli_stmt | mysqli_warning | stdClass | streamWrapper | tidy |\n  tidyNode\n)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620745621562,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925366759751738,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }