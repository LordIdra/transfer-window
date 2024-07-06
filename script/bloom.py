import sys,cv2

path = sys.argv[1]

print(path)

original_image = cv2.imread(path)
blur_image = cv2.GaussianBlur(original_image, (17,17), 0)
bitmap = wx.Bitmap.FromBufferRGBA(blurred_image.size[0], blurred_image.size[1], blurred_image.tobytes())
image = cv2.addWeighted(original_image, 1.0, blur_image, 1.0, 0.0)
cv2.imwrite(path, image)