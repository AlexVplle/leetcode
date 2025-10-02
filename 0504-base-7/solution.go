import (
    "strings"
    "strconv"
)

func convertToBase7(num int) string {
    return string(strconv.FormatInt(int64(num), 7))
}
